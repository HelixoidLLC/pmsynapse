import * as vscode from "vscode";
import { WebviewPanelHost } from "./panels/WebviewPanelHost";
import { registerCommands } from "./commands";

let playgroundPanel: WebviewPanelHost | undefined;

export function activate(context: vscode.ExtensionContext) {
  console.log("PMSynapse extension is now active");

  // Register commands
  registerCommands(context);

  // Register the playground panel command
  const openPlaygroundCommand = vscode.commands.registerCommand(
    "pmsynapse.openPlayground",
    () => {
      if (playgroundPanel) {
        playgroundPanel.reveal();
      } else {
        playgroundPanel = new WebviewPanelHost(context.extensionUri, context);
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
          vscode.window.showInformationMessage(
            `Created ${itemType}: ${title}`
          );
          // TODO: Send to backend via RPC
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
        vscode.commands.executeCommand("pmsynapse.openPlayground");
      }
    }
  );

  context.subscriptions.push(
    openPlaygroundCommand,
    createIdlcItemCommand,
    showKnowledgeGraphCommand
  );

  // Register tree data providers
  const idlcTreeProvider = new IdlcTreeProvider();
  vscode.window.registerTreeDataProvider("pmsynapse.idlcItems", idlcTreeProvider);

  const graphTreeProvider = new KnowledgeGraphTreeProvider();
  vscode.window.registerTreeDataProvider("pmsynapse.knowledgeGraph", graphTreeProvider);
}

export function deactivate() {
  if (playgroundPanel) {
    playgroundPanel.dispose();
  }
}

// Tree data provider for IDLC items
class IdlcTreeProvider implements vscode.TreeDataProvider<IdlcItem> {
  private _onDidChangeTreeData = new vscode.EventEmitter<IdlcItem | undefined>();
  readonly onDidChangeTreeData = this._onDidChangeTreeData.event;

  refresh(): void {
    this._onDidChangeTreeData.fire(undefined);
  }

  getTreeItem(element: IdlcItem): vscode.TreeItem {
    return element;
  }

  getChildren(_element?: IdlcItem): Thenable<IdlcItem[]> {
    // TODO: Fetch from backend
    return Promise.resolve([
      new IdlcItem("Sample Idea", "idea", vscode.TreeItemCollapsibleState.None),
      new IdlcItem("Sample Feature", "feature", vscode.TreeItemCollapsibleState.None),
    ]);
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

  refresh(): void {
    this._onDidChangeTreeData.fire(undefined);
  }

  getTreeItem(element: GraphNode): vscode.TreeItem {
    return element;
  }

  getChildren(_element?: GraphNode): Thenable<GraphNode[]> {
    // TODO: Fetch from backend
    return Promise.resolve([
      new GraphNode("Ideas", "category", vscode.TreeItemCollapsibleState.Collapsed),
      new GraphNode("Features", "category", vscode.TreeItemCollapsibleState.Collapsed),
      new GraphNode("Tasks", "category", vscode.TreeItemCollapsibleState.Collapsed),
    ]);
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
