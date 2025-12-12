import * as vscode from "vscode";
import { getUri } from "../utils/getUri";
import { getNonce } from "../utils/getNonce";

/**
 * WebviewPanelHost manages the webview panel that hosts the PMSynapse UI.
 *
 * This follows the BAML pattern for dual deployment:
 * - In development: loads from Vite dev server (hot reload)
 * - In production: loads bundled assets with CSP
 */
export class WebviewPanelHost {
  public static readonly viewType = "pmsynapse.playground";

  private readonly _panel: vscode.WebviewPanel;
  private readonly _extensionUri: vscode.Uri;
  private _disposables: vscode.Disposable[] = [];
  private _onDidDispose: (() => void) | undefined;

  constructor(extensionUri: vscode.Uri, context: vscode.ExtensionContext) {
    this._extensionUri = extensionUri;

    // Create the webview panel
    this._panel = vscode.window.createWebviewPanel(
      WebviewPanelHost.viewType,
      "PMSynapse",
      vscode.ViewColumn.One,
      {
        enableScripts: true,
        retainContextWhenHidden: true,
        localResourceRoots: [
          vscode.Uri.joinPath(extensionUri, "dist"),
          vscode.Uri.joinPath(extensionUri, "webview-ui", "dist"),
        ],
      }
    );

    // Set the HTML content
    this._panel.webview.html = this._getWebviewContent();

    // Handle messages from the webview
    this._panel.webview.onDidReceiveMessage(
      (message) => this._handleMessage(message),
      null,
      this._disposables
    );

    // Handle panel disposal
    this._panel.onDidDispose(
      () => this._dispose(),
      null,
      this._disposables
    );
  }

  /**
   * Register a callback for when the panel is disposed
   */
  public onDidDispose(callback: () => void): void {
    this._onDidDispose = callback;
  }

  /**
   * Reveal the panel
   */
  public reveal(): void {
    this._panel.reveal();
  }

  /**
   * Post a message to the webview
   */
  public postMessage(message: unknown): void {
    this._panel.webview.postMessage(message);
  }

  /**
   * Dispose of the panel
   */
  public dispose(): void {
    this._panel.dispose();
  }

  private _dispose(): void {
    // Clean up resources
    while (this._disposables.length) {
      const disposable = this._disposables.pop();
      if (disposable) {
        disposable.dispose();
      }
    }

    // Call the dispose callback
    if (this._onDidDispose) {
      this._onDidDispose();
    }
  }

  /**
   * Handle messages from the webview
   */
  private _handleMessage(message: { type: string; payload?: unknown }): void {
    switch (message.type) {
      case "ready":
        console.log("PMSynapse webview is ready");
        break;

      case "createIdlcItem":
        vscode.commands.executeCommand("pmsynapse.createIdlcItem");
        break;

      case "showKnowledgeGraph":
        // Handle knowledge graph display
        break;

      case "executeCommand":
        if (typeof message.payload === "string") {
          vscode.commands.executeCommand(message.payload);
        }
        break;

      case "showMessage":
        if (typeof message.payload === "string") {
          vscode.window.showInformationMessage(message.payload);
        }
        break;

      case "error":
        if (typeof message.payload === "string") {
          vscode.window.showErrorMessage(message.payload);
        }
        break;

      default:
        console.log("Unknown message type:", message.type);
    }
  }

  /**
   * Get the HTML content for the webview
   */
  private _getWebviewContent(): string {
    const webview = this._panel.webview;
    const nonce = getNonce();

    // Check if we're in development mode
    const isDev = process.env.VSCODE_DEBUG_MODE === "true";

    if (isDev) {
      // Development mode: load from Vite dev server
      return this._getDevContent(nonce);
    }

    // Production mode: load bundled assets
    return this._getProdContent(webview, nonce);
  }

  /**
   * Get development mode HTML (loads from Vite dev server)
   */
  private _getDevContent(nonce: string): string {
    const devServerUrl = "http://localhost:3030";

    return `<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>PMSynapse</title>
</head>
<body>
  <div id="root"></div>
  <script type="module" nonce="${nonce}">
    import RefreshRuntime from "${devServerUrl}/@react-refresh";
    RefreshRuntime.injectIntoGlobalHook(window);
    window.$RefreshReg$ = () => {};
    window.$RefreshSig$ = () => (type) => type;
    window.__vite_plugin_react_preamble_installed__ = true;
  </script>
  <script type="module" src="${devServerUrl}/@vite/client"></script>
  <script type="module" src="${devServerUrl}/src/main.tsx"></script>
</body>
</html>`;
  }

  /**
   * Get production mode HTML (loads bundled assets)
   */
  private _getProdContent(webview: vscode.Webview, nonce: string): string {
    // Get URIs for bundled assets
    const stylesUri = getUri(webview, this._extensionUri, [
      "webview-ui",
      "dist",
      "assets",
      "index.css",
    ]);
    const scriptUri = getUri(webview, this._extensionUri, [
      "webview-ui",
      "dist",
      "assets",
      "index.js",
    ]);

    // Content Security Policy
    const csp = [
      `default-src 'none'`,
      `style-src ${webview.cspSource} 'unsafe-inline'`,
      `script-src 'nonce-${nonce}'`,
      `font-src ${webview.cspSource}`,
      `img-src ${webview.cspSource} https: data:`,
      `connect-src https:`,
    ].join("; ");

    return `<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <meta http-equiv="Content-Security-Policy" content="${csp}">
  <link rel="stylesheet" type="text/css" href="${stylesUri}">
  <title>PMSynapse</title>
</head>
<body>
  <div id="root"></div>
  <script type="module" nonce="${nonce}" src="${scriptUri}"></script>
</body>
</html>`;
  }
}
