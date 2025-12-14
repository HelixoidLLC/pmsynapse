import * as vscode from "vscode";
import { WebviewPanelHost } from "./panels/WebviewPanelHost";
import { registerCommands } from "./commands";
import { DaemonClient } from "./daemon-client";

let playgroundPanel: WebviewPanelHost | undefined;
let daemonClient: DaemonClient | undefined;
let outputChannel: vscode.OutputChannel;

export async function activate(context: vscode.ExtensionContext) {
  // Create output channel
  outputChannel = vscode.window.createOutputChannel("PMSynapse");
  context.subscriptions.push(outputChannel);

  outputChannel.appendLine("PMSynapse extension is now active");
  outputChannel.show();

  // Initialize daemon client
  try {
    outputChannel.appendLine("Connecting to daemon...");
    daemonClient = await DaemonClient.get_instance();
    outputChannel.appendLine("✓ Daemon client connected successfully");
  } catch (error) {
    const errorMsg = `Failed to connect to PMSynapse daemon: ${error}`;
    outputChannel.appendLine(`✗ ${errorMsg}`);
    vscode.window.showErrorMessage(errorMsg);
    // Continue activation even if daemon fails
  }

  // Register tree data providers first (so they're available to commands)
  const idlcTreeProvider = new IdlcTreeProvider(daemonClient);
  vscode.window.registerTreeDataProvider("pmsynapse.idlcItems", idlcTreeProvider);

  const graphTreeProvider = new KnowledgeGraphTreeProvider(daemonClient);
  vscode.window.registerTreeDataProvider("pmsynapse.knowledgeGraph", graphTreeProvider);

  // Register commands
  registerCommands(context);

  // Register the dashboard panel command
  const openDashboardCommand = vscode.commands.registerCommand(
    "pmsynapse.openDashboard",
    () => {
      if (playgroundPanel) {
        playgroundPanel.reveal();
      } else {
        playgroundPanel = new WebviewPanelHost(
          context.extensionUri,
          context,
          daemonClient,
          () => {
            // Refresh both tree views when nodes change
            outputChannel.appendLine("Refreshing tree views from webview callback");
            idlcTreeProvider.refresh();
            graphTreeProvider.refresh();
          }
        );
        playgroundPanel.onDidDispose(() => {
          playgroundPanel = undefined;
        });
      }
    }
  );

  // Register IDLC item creation command
  const createIdlcItemCommand = vscode.commands.registerCommand(
    "pmsynapse.createIdlcItem",
    async () => {
      const itemType = await vscode.window.showQuickPick(
        ["Idea", "Feature", "Task", "Decision", "Question", "Assumption"],
        { placeHolder: "Select IDLC item type" }
      );

      if (itemType) {
        const title = await vscode.window.showInputBox({
          prompt: `Enter ${itemType} title`,
          placeHolder: `My new ${itemType.toLowerCase()}`,
        });

        if (title) {
          if (daemonClient) {
            try {
              outputChannel.appendLine(`Creating ${itemType}: ${title}`);
              const node = await daemonClient.nodes.create({
                nodeType: itemType.toLowerCase() as any,
                title,
                content: '',
              });
              outputChannel.appendLine(`✓ Created node: ${JSON.stringify(node)}`);
              vscode.window.showInformationMessage(
                `Created ${itemType}: ${title}`
              );
              // Refresh both tree views
              idlcTreeProvider.refresh();
              graphTreeProvider.refresh();
            } catch (error) {
              outputChannel.appendLine(`✗ Failed to create ${itemType}: ${error}`);
              vscode.window.showErrorMessage(
                `Failed to create ${itemType}: ${error}`
              );
            }
          } else {
            outputChannel.appendLine("✗ Daemon not connected");
            vscode.window.showWarningMessage(
              "Daemon not connected. Item not saved."
            );
          }
        }
      }
    }
  );

  // Register knowledge graph command
  const showKnowledgeGraphCommand = vscode.commands.registerCommand(
    "pmsynapse.showKnowledgeGraph",
    () => {
      if (playgroundPanel) {
        playgroundPanel.postMessage({ type: "showKnowledgeGraph" });
      } else {
        vscode.commands.executeCommand("pmsynapse.openDashboard");
      }
    }
  );

  context.subscriptions.push(
    openDashboardCommand,
    createIdlcItemCommand,
    showKnowledgeGraphCommand
  );
}

export function deactivate() {
  if (playgroundPanel) {
    playgroundPanel.dispose();
  }

  if (daemonClient) {
    daemonClient.dispose();
  }
}

// Tree data provider for IDLC items
class IdlcTreeProvider implements vscode.TreeDataProvider<IdlcItem> {
  private _onDidChangeTreeData = new vscode.EventEmitter<IdlcItem | undefined>();
  readonly onDidChangeTreeData = this._onDidChangeTreeData.event;

  constructor(private client: DaemonClient | undefined) {}

  refresh(): void {
    console.log("IdlcTreeProvider.refresh() called");
    this._onDidChangeTreeData.fire(undefined);
  }

  getTreeItem(element: IdlcItem): vscode.TreeItem {
    return element;
  }

  async getChildren(_element?: IdlcItem): Promise<IdlcItem[]> {
    if (!this.client) {
      return [
        new IdlcItem("Daemon not connected", "error", vscode.TreeItemCollapsibleState.None),
      ];
    }

    try {
      const nodes = await this.client.nodes.list();
      return nodes.map(
        (node) =>
          new IdlcItem(node.title, node.nodeType, vscode.TreeItemCollapsibleState.None)
      );
    } catch (error) {
      console.error("Failed to fetch nodes:", error);
      return [
        new IdlcItem("Failed to load items", "error", vscode.TreeItemCollapsibleState.None),
      ];
    }
  }
}

class IdlcItem extends vscode.TreeItem {
  constructor(
    public readonly label: string,
    public readonly itemType: string,
    public readonly collapsibleState: vscode.TreeItemCollapsibleState
  ) {
    super(label, collapsibleState);
    this.tooltip = `${itemType}: ${label}`;
    this.description = itemType;
    this.iconPath = new vscode.ThemeIcon(this.getIcon());
  }

  private getIcon(): string {
    switch (this.itemType) {
      case "idea":
        return "lightbulb";
      case "feature":
        return "package";
      case "task":
        return "tasklist";
      case "decision":
        return "check";
      case "question":
        return "question";
      default:
        return "circle-outline";
    }
  }
}

// Tree data provider for knowledge graph
class KnowledgeGraphTreeProvider implements vscode.TreeDataProvider<GraphNode> {
  private _onDidChangeTreeData = new vscode.EventEmitter<GraphNode | undefined>();
  readonly onDidChangeTreeData = this._onDidChangeTreeData.event;

  constructor(private client: DaemonClient | undefined) {}

  refresh(): void {
    console.log("KnowledgeGraphTreeProvider.refresh() called");
    this._onDidChangeTreeData.fire(undefined);
  }

  getTreeItem(element: GraphNode): vscode.TreeItem {
    return element;
  }

  async getChildren(element?: GraphNode): Promise<GraphNode[]> {
    if (!this.client) {
      return [
        new GraphNode("Daemon not connected", "error", vscode.TreeItemCollapsibleState.None),
      ];
    }

    try {
      const nodes = await this.client.nodes.list();

      // If no element, show grouped types (root level)
      if (!element) {
        const typeMap = new Map<string, number>();
        nodes.forEach((node) => {
          const count = typeMap.get(node.nodeType) || 0;
          typeMap.set(node.nodeType, count + 1);
        });

        return Array.from(typeMap.entries()).map(
          ([type, count]) =>
            new GraphNode(
              `${type} (${count})`,
              type,
              vscode.TreeItemCollapsibleState.Collapsed
            )
        );
      }

      // If element provided, show nodes of that type
      const filteredNodes = nodes.filter((node) => node.nodeType === element.nodeType);
      return filteredNodes.map(
        (node) =>
          new GraphNode(node.title, node.nodeType, vscode.TreeItemCollapsibleState.None)
      );
    } catch (error) {
      console.error("Failed to fetch graph:", error);
      return [
        new GraphNode("Failed to load graph", "error", vscode.TreeItemCollapsibleState.None),
      ];
    }
  }
}

class GraphNode extends vscode.TreeItem {
  constructor(
    public readonly label: string,
    public readonly nodeType: string,
    public readonly collapsibleState: vscode.TreeItemCollapsibleState
  ) {
    super(label, collapsibleState);
    this.tooltip = label;
    this.iconPath = new vscode.ThemeIcon("type-hierarchy");
  }
}
