// Session Viewer JavaScript - included in HTML template
function renderSession(session) {
  const container = document.getElementById("threadContainer");
  const nav = document.getElementById("messageNav");
  const messages = session.messages.filter(m => m.message_type === "user" || m.message_type === "assistant");
  nav.innerHTML = messages.map((_, i) => '<button class="nav-dot" onclick="scrollToMessage(' + i + ')"><span class="dot"></span><span class="line"></span></button>').join("");
  container.innerHTML = messages.map((msg, i) => renderMessage(msg, i)).join("");
  const toolUsage = {};
  session.messages.forEach(msg => { msg.tool_uses.forEach(tool => { toolUsage[tool.tool_name] = (toolUsage[tool.tool_name] || 0) + 1; }); });
  const toolUsageEl = document.getElementById("toolUsage");
  if (toolUsageEl) { toolUsageEl.innerHTML = Object.entries(toolUsage).sort((a, b) => b[1] - a[1]).map(([name, count]) => '<div class="tool-usage-item"><span class="name">' + name + '</span><span class="count">' + count + '</span></div>').join(""); }
}

function renderMessage(message, index) {
  const isUser = message.message_type === "user";
  return '<div class="message-block" id="message-' + index + '"><a href="#message-' + index + '" class="block-link">#</a>' + (isUser ? renderUserMessage(message) : renderAssistantMessage(message, index)) + '</div>';
}

function renderUserMessage(message) {
  const files = extractFileReferences(message.content || "");
  let html = '<div class="user-message"><div class="avatar"><svg viewBox="0 0 24 24" width="12" height="12" fill="none" stroke="currentColor" stroke-width="2"><path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"/><circle cx="12" cy="7" r="4"/></svg></div><div class="content">';
  if (files.length > 0) {
    html += '<div class="file-chips">' + files.map(f => '<span class="file-chip"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><polyline points="14 2 14 8 20 8"/></svg>' + f.split("/").pop() + '</span>').join("") + '</div>';
  }
  html += escapeHtml(message.content || "") + '</div></div>';
  return html;
}

function renderAssistantMessage(message, blockIndex) {
  const hasThinking = Boolean(message.thinking);
  const hasText = Boolean(message.content);
  const hasTools = message.tool_uses.length > 0;
  let html = '<div class="assistant-message">';
  if (hasThinking) html += renderThinking(message.thinking, blockIndex);
  if (hasText) html += '<div class="markdown">' + renderMarkdown(message.content) + '</div>';
  if (hasTools) html += message.tool_uses.map((t, i) => renderToolUse(t, blockIndex + "-" + i)).join("");
  html += '</div>';
  return html;
}

function renderThinking(content, blockIndex) {
  const thinkingId = "thinking-" + blockIndex;
  return '<div class="thinking-block"><button class="thinking-trigger" onclick="toggleThinking(\'' + thinkingId + '\')" aria-expanded="false"><svg viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="2"><polyline points="9 18 15 12 9 6"/></svg><svg viewBox="0 0 24 24" width="14" height="14" fill="none" stroke="currentColor" stroke-width="2"><path d="M12 5a3 3 0 1 0-5.997.125 4 4 0 0 0-2.526 5.77 4 4 0 0 0 .556 6.588A4 4 0 1 0 12 18Z"/><path d="M12 5a3 3 0 1 1 5.997.125 4 4 0 0 1 2.526 5.77 4 4 0 0 1-.556 6.588A4 4 0 1 1 12 18Z"/></svg>Thinking</button><div class="thinking-content" id="' + thinkingId + '"><div class="markdown">' + renderMarkdown(content) + '</div></div></div>';
}

function renderToolUse(tool, toolId) {
  const summary = getToolSummary(tool);
  const filePath = tool.input.file_path || tool.input.path;
  const isFileOp = ["Read", "Write", "Edit"].includes(tool.tool_name);
  let html = '<div class="tool-chip" onclick="toggleToolDetails(\'tool-' + toolId + '\')">' + getToolIcon(tool.tool_name);
  if (isFileOp && filePath) {
    const parts = filePath.split("/");
    html += '<span class="file-path"><span class="path-prefix">' + parts.slice(0, -1).join("/") + '</span><span class="path-separator">/</span><span>' + parts.pop() + '</span></span>';
  } else {
    html += '<span class="tool-name">' + summary + '</span>';
  }
  if (tool.output) {
    html += '<span class="tool-status ' + (tool.output.success ? "success" : "error") + '">' + (tool.output.success ? "\u2713" : "\u2717") + '</span>';
  }
  html += '</div><div class="tool-details" id="tool-' + toolId + '"><div class="tool-details-label">Input</div><div class="code-block"><pre><code>' + escapeHtml(JSON.stringify(tool.input, null, 2)) + '</code></pre></div>';
  if (tool.output) {
    html += '<div class="tool-details-label" style="margin-top: 0.75rem;">Output</div>';
    if (tool.output.error) {
      html += '<div style="padding: 0.75rem; background: hsl(0 40% 95%); border-radius: 0.25rem; color: hsl(0 50% 40%);">' + escapeHtml(tool.output.error) + '</div>';
    } else {
      const content = typeof tool.output.content === "string" ? tool.output.content.slice(0, 2000) : JSON.stringify(tool.output.content, null, 2);
      html += '<div class="code-block"><pre><code>' + escapeHtml(content) + '</code></pre></div>';
    }
  }
  html += '</div>';
  return html;
}

function renderMarkdown(text) {
  if (!text) return "";
  text = text.replace(/```(\w*)\n([\s\S]*?)```/g, function(_, lang, code) { return '<div class="code-block"><pre><code>' + escapeHtml(code.trim()) + '</code></pre></div>'; });
  text = text.replace(/^### (.+)$/gm, '<h3>$1</h3>');
  text = text.replace(/^## (.+)$/gm, '<h2>$1</h2>');
  text = text.replace(/^# (.+)$/gm, '<h1>$1</h1>');
  text = text.replace(/\*\*(.+?)\*\*/g, '<strong>$1</strong>');
  text = text.replace(/`([^`]+)`/g, '<code>$1</code>');
  text = text.replace(/^- (.+)$/gm, '<li>$1</li>');
  text = text.replace(/(<li>.*<\/li>\n?)+/g, '<ul>$&</ul>');
  var paras = text.split("\n\n");
  return paras.map(function(p) { if (p.startsWith("<") || p.trim() === "") return p; return '<p>' + p + '</p>'; }).join("");
}

function getToolSummary(tool) {
  var input = tool.input;
  switch (tool.tool_name) {
    case "Read": return "Read " + (input.file_path || "").split("/").pop();
    case "Write": return "Write " + (input.file_path || "").split("/").pop();
    case "Edit": return "Edit " + (input.file_path || "").split("/").pop();
    case "Bash": return (input.command || "").slice(0, 50);
    case "Grep": return 'Search "' + input.pattern + '"';
    case "Glob": return "Find " + input.pattern;
    case "Task": return input.description || "Running task...";
    case "TodoWrite": return "Updated TODOs";
    default: return tool.tool_name;
  }
}

function getToolIcon(name) {
  var file = '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><polyline points="14 2 14 8 20 8"/></svg>';
  var icons = {
    Read: file, Write: file, Edit: file,
    Bash: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="4 17 10 11 4 5"/><line x1="12" y1="19" x2="20" y2="19"/></svg>',
    Grep: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/></svg>',
    Glob: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/></svg>',
    Task: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M12 8V4H8"/><rect x="8" y="2" width="8" height="4" rx="1" ry="1"/><path d="M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2"/></svg>',
    TodoWrite: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="8" y1="6" x2="21" y2="6"/><line x1="8" y1="12" x2="21" y2="12"/><line x1="8" y1="18" x2="21" y2="18"/><line x1="3" y1="6" x2="3.01" y2="6"/><line x1="3" y1="12" x2="3.01" y2="12"/><line x1="3" y1="18" x2="3.01" y2="18"/></svg>'
  };
  return icons[name] || '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M14.7 6.3a1 1 0 0 0 0 1.4l1.6 1.6a1 1 0 0 0 1.4 0l3.77-3.77a6 6 0 0 1-7.94 7.94l-6.91 6.91a2.12 2.12 0 0 1-3-3l6.91-6.91a6 6 0 0 1 7.94-7.94l-3.76 3.76z"/></svg>';
}

function extractFileReferences(text) {
  var matches = text.match(/@[\w./\-]+|file:\/\/[^\s]+/g) || [];
  return matches.map(function(m) { return m.replace(/^@|^file:\/\//, ""); });
}

function escapeHtml(text) {
  var div = document.createElement("div");
  div.textContent = text;
  return div.innerHTML;
}

function scrollToMessage(index) {
  var el = document.getElementById("message-" + index);
  if (el) el.scrollIntoView({ behavior: "smooth" });
}

function toggleThinking(id) {
  var el = document.getElementById(id);
  var btn = el.previousElementSibling;
  var isExpanded = el.classList.contains("expanded");
  el.classList.toggle("expanded");
  btn.setAttribute("aria-expanded", !isExpanded);
}

function toggleToolDetails(id) {
  var el = document.getElementById(id);
  el.classList.toggle("expanded");
}

document.addEventListener("DOMContentLoaded", function() {
  if (typeof sessionData !== "undefined" && sessionData) {
    renderSession(sessionData);
  }
});
