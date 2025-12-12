import * as vscode from "vscode";

/**
 * Register all extension commands
 */
export function registerCommands(context: vscode.ExtensionContext): void {
  // Commands are registered in extension.ts for now
  // This module can be expanded for additional command registrations

  // Example: Register a command to open settings
  const openSettingsCommand = vscode.commands.registerCommand(
    "pmsynapse.openSettings",
    () => {
      vscode.commands.executeCommand(
        "workbench.action.openSettings",
        "pmsynapse"
      );
    }
  );

  context.subscriptions.push(openSettingsCommand);
}
