import * as vscode from 'vscode';
import * as fs from 'fs';
import * as path from 'path';
import * as os from 'os';
import { spawn, ChildProcess } from 'child_process';
import { PMSynapseClient, type EventHandlers } from '@pmsynapse/api-client';

export class DaemonClient extends PMSynapseClient {
  private static instance: DaemonClient | null = null;
  private daemon_process: ChildProcess | null = null;

  private constructor(port: number) {
    super({ port });
  }

  static async get_instance(): Promise<DaemonClient> {
    if (DaemonClient.instance) {
      return DaemonClient.instance;
    }

    const port = await DaemonClient.find_or_start_daemon();
    DaemonClient.instance = new DaemonClient(port);

    // Verify health
    const is_healthy = await DaemonClient.instance.health();
    if (!is_healthy) {
      throw new Error('Daemon health check failed');
    }

    return DaemonClient.instance;
  }

  private static async find_or_start_daemon(): Promise<number> {
    // Try to find existing daemon
    const existing_port = await DaemonClient.get_daemon_port();
    if (existing_port) {
      // Verify it's actually running
      const test_client = new PMSynapseClient({ port: existing_port });
      const is_healthy = await test_client.health();
      if (is_healthy) {
        return existing_port;
      }
    }

    // Start new daemon
    return await DaemonClient.start_daemon();
  }

  private static async get_daemon_port(): Promise<number | null> {
    const pid_path = DaemonClient.get_pid_file_path();

    if (!fs.existsSync(pid_path)) {
      return null;
    }

    try {
      const content = fs.readFileSync(pid_path, 'utf-8').trim();

      // Handle "pid:port" format
      if (content.includes(':')) {
        const parts = content.split(':');
        return parseInt(parts[1], 10);
      }

      return null;
    } catch {
      return null;
    }
  }

  private static get_pid_file_path(): string {
    const home = os.homedir();
    return path.join(home, '.pmsynapse', 'daemon.pid');
  }

  private static async start_daemon(): Promise<number> {
    return new Promise((resolve, reject) => {
      // Find snps CLI binary
      const snps_path = DaemonClient.find_snps_binary();
      if (!snps_path) {
        reject(new Error('snps CLI not found. Please ensure it is installed and in PATH.'));
        return;
      }

      // Spawn daemon in foreground mode with dynamic port
      const child = spawn(snps_path, ['daemon', 'start', '--foreground', '--port', '0'], {
        stdio: ['ignore', 'pipe', 'pipe'],
        detached: true,
      });

      let stdout_data = '';
      let stderr_data = '';

      child.stdout?.on('data', (data) => {
        stdout_data += data.toString();

        // Look for HTTP_PORT=XXXXX
        const port_match = stdout_data.match(/HTTP_PORT=(\d+)/);
        if (port_match) {
          const port = parseInt(port_match[1], 10);

          // Wait a bit for server to be ready
          setTimeout(async () => {
            const test_client = new PMSynapseClient({ port });
            let retries = 10;

            while (retries > 0) {
              try {
                const is_healthy = await test_client.health();
                if (is_healthy) {
                  resolve(port);
                  return;
                }
              } catch {
                // Not ready yet
              }

              await new Promise(r => setTimeout(r, 100));
              retries--;
            }

            reject(new Error('Daemon started but health check failed'));
          }, 500);
        }
      });

      child.stderr?.on('data', (data) => {
        stderr_data += data.toString();
      });

      child.on('error', (err) => {
        reject(new Error(`Failed to start daemon: ${err.message}`));
      });

      child.on('exit', (code) => {
        if (code !== 0 && code !== null) {
          reject(new Error(`Daemon exited with code ${code}: ${stderr_data}`));
        }
      });

      // Timeout after 10 seconds
      setTimeout(() => {
        reject(new Error('Timeout waiting for daemon to start'));
      }, 10000);
    });
  }

  private static find_snps_binary(): string | null {
    // Try common locations
    const possible_paths = [
      'snps', // In PATH
      path.join(os.homedir(), '.cargo', 'bin', 'snps'), // Cargo install
    ];

    for (const bin_path of possible_paths) {
      try {
        // Check if executable exists
        if (fs.existsSync(bin_path)) {
          return bin_path;
        }
      } catch {
        // Continue to next
      }
    }

    // Try finding in PATH
    try {
      const which_result = spawn('which', ['snps'], { stdio: 'pipe' });
      return 'snps'; // Available in PATH
    } catch {
      return null;
    }
  }

  dispose(): void {
    super.disconnect();

    if (this.daemon_process) {
      this.daemon_process.kill();
      this.daemon_process = null;
    }
  }
}
