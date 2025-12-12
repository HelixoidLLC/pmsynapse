/**
 * Generates a random nonce for Content Security Policy.
 *
 * A nonce (number used once) is a random string that allows inline scripts
 * to be executed in the webview while maintaining security.
 *
 * @returns A 32-character random string
 */
export function getNonce(): string {
  let text = "";
  const possible =
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
  for (let i = 0; i < 32; i++) {
    text += possible.charAt(Math.floor(Math.random() * possible.length));
  }
  return text;
}
