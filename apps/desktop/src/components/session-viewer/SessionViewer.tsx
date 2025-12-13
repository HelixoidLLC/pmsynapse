import React, { useState, useMemo } from 'react';
import { cn } from '@/lib/utils';
import {
  ChevronRight,
  Clock,
  GitBranch,
  MessageSquare,
  Wrench,
  Brain,
  User,
  Bot,
  Copy,
  Check,
  Folder,
  FileText,
  Link2,
  Terminal,
  Search,
  Files,
  ListTodo,
} from 'lucide-react';
import { Button } from '@/components/ui/button';
import './session-viewer.css';

// Types matching Session from snps-core models.rs
interface ToolResult {
  success: boolean;
  content: unknown;
  error?: string;
  execution_time_ms?: number;
}

interface ToolUse {
  tool_id: string;
  tool_name: string;
  timestamp?: string;
  input: Record<string, unknown>;
  output?: ToolResult;
}

interface MessageContent {
  text?: string;
  thinking?: string;
  raw_content: unknown;
}

interface Message {
  uuid: string;
  parent_uuid?: string;
  is_sidechain: boolean;
  message_type: 'user' | 'assistant' | 'system' | 'summary' | 'file-history-snapshot';
  role?: 'user' | 'assistant' | 'system';
  timestamp?: string;
  content: MessageContent;
  tool_uses: ToolUse[];
}

interface SessionMetadata {
  cwd?: string;
  version?: string;
  git_branch?: string;
  start_time?: string;
  end_time?: string;
  duration_seconds?: number;
  message_count: number;
  tool_call_count: number;
  file_size_bytes: number;
}

interface Session {
  session_id: string;
  is_agent: boolean;
  agent_id?: string;
  parent_session_id?: string;
  metadata: SessionMetadata;
  messages: Message[];
  child_agents: string[];
}

interface SessionViewerProps {
  session: Session;
  className?: string;
  title?: string;
  author?: string;
}

// Main Session Viewer Component
export function SessionViewer({ session, className, title, author }: SessionViewerProps) {
  const displayMessages = useMemo(() =>
    session.messages.filter((msg) =>
      msg.message_type === 'user' || msg.message_type === 'assistant'
    ),
    [session.messages]
  );

  const sessionTitle = title || extractTitle(session);

  return (
    <div className={cn('session-viewer w-full max-w-2xl mx-auto px-4', className)}>
      {/* Navigation dots */}
      <MessageNavigation messages={displayMessages} />

      {/* Header */}
      <SessionHeader
        session={session}
        title={sessionTitle}
        author={author}
      />

      {/* Thread container */}
      <div className="thread-container">
        {displayMessages.map((message, index) => (
          <MessageBlock
            key={message.uuid}
            message={message}
            blockIndex={index}
          />
        ))}
      </div>

      {/* Statistics */}
      <SessionStats session={session} />
    </div>
  );
}

// Extract title from first user message
function extractTitle(session: Session): string {
  const firstUserMessage = session.messages.find(m => m.message_type === 'user');
  if (firstUserMessage?.content.text) {
    const text = firstUserMessage.content.text;
    // Take first line or first 80 chars
    const firstLine = text.split('\n')[0];
    return firstLine.length > 80 ? firstLine.slice(0, 77) + '...' : firstLine;
  }
  return `Session ${session.session_id.slice(0, 8)}`;
}

// Message Navigation (left dots)
function MessageNavigation({ messages }: { messages: Message[] }) {
  return (
    <div className="message-nav">
      <div className="message-nav-inner">
        {messages.map((msg, index) => (
          <button
            key={msg.uuid}
            type="button"
            className="message-nav-dot group"
            aria-label={`Jump to message ${index + 1}`}
            onClick={() => {
              document.getElementById(`message-${index}`)?.scrollIntoView({
                behavior: 'smooth'
              });
            }}
          >
            <div className="dot" />
            <div className="line" />
          </button>
        ))}
      </div>
    </div>
  );
}

// Session Header Component
function SessionHeader({ session, title, author }: {
  session: Session;
  title: string;
  author?: string;
}) {
  const formattedDate = session.metadata.start_time
    ? new Date(session.metadata.start_time).toLocaleDateString('en-US', {
        year: 'numeric',
        month: 'short',
        day: 'numeric',
      })
    : 'Unknown date';

  return (
    <div className="session-header">
      <div className="flex items-baseline justify-start sm:justify-center w-full">
        <h1 className="session-title">{title}</h1>
      </div>

      <div className="session-author">
        {/* Avatar */}
        <div className="user-message-avatar">
          <span className="bg-muted flex size-full items-center justify-center text-center">
            <span className="text-[10px] font-medium">
              {(author || 'User').slice(0, 2).toUpperCase()}
            </span>
          </span>
        </div>

        <div className="flex items-center gap-1.5">
          <span className="session-author-name">{author || 'User'}</span>
          <span className="session-author-separator">•</span>
          <span className="session-author-date">{formattedDate}</span>
          {session.metadata.git_branch && (
            <>
              <span className="session-author-separator">•</span>
              <span className="text-muted-foreground text-sm flex items-center gap-1">
                <GitBranch className="h-3 w-3" />
                {session.metadata.git_branch}
              </span>
            </>
          )}
        </div>
      </div>
    </div>
  );
}

// Message Block Component
function MessageBlock({ message, blockIndex }: { message: Message; blockIndex: number }) {
  const isUser = message.message_type === 'user';

  return (
    <div
      id={`message-${blockIndex}`}
      className="message-block group"
    >
      {/* Block link icon */}
      <a
        href={`#message-${blockIndex}`}
        className="block-link-icon"
        aria-label="Link to this block"
        title="Link to this block"
      >
        <Link2 className="w-2.5 h-2.5" />
      </a>

      {isUser ? (
        <UserMessage message={message} />
      ) : (
        <AssistantMessage message={message} blockIndex={blockIndex} />
      )}
    </div>
  );
}

// User Message Component
function UserMessage({ message }: { message: Message }) {
  const attachedFiles = extractFileReferences(message.content.text || '');

  return (
    <div className="flex items-start gap-2">
      {/* Avatar */}
      <div className="user-message-avatar">
        <span className="bg-muted flex size-full items-center justify-center text-center hover:bg-muted-foreground/5 transition-colors">
          <User className="h-3 w-3" />
        </span>
      </div>

      {/* Message content */}
      <section className="user-message flex-1" aria-label="User message">
        {/* Attached files */}
        {attachedFiles.length > 0 && (
          <div className="flex flex-wrap pb-1 gap-1">
            {attachedFiles.map((file, index) => (
              <FileChip key={index} path={file} />
            ))}
          </div>
        )}

        {/* Text content */}
        {message.content.text && (
          <div className="w-full text-left overflow-x-auto break-words whitespace-pre-wrap">
            {message.content.text}
          </div>
        )}
      </section>
    </div>
  );
}

// File Chip Component
function FileChip({ path }: { path: string }) {
  const fileName = path.split('/').pop() || path;

  return (
    <span className="file-chip" title={path}>
      <FileText className="h-3 w-3" />
      <span className="truncate">{fileName}</span>
    </span>
  );
}

// Assistant Message Component
function AssistantMessage({ message, blockIndex }: { message: Message; blockIndex: number }) {
  const hasThinking = Boolean(message.content.thinking);
  const hasText = Boolean(message.content.text);
  const hasToolUses = message.tool_uses.length > 0;

  return (
    <div className="flex flex-col gap-1">
      {/* Thinking block */}
      {hasThinking && (
        <ThinkingBlock
          content={message.content.thinking!}
          blockId={`message-${blockIndex}-thinking`}
        />
      )}

      {/* Text content */}
      {hasText && (
        <div className="markdown-content">
          <MarkdownRenderer content={message.content.text!} />
        </div>
      )}

      {/* Tool uses */}
      {hasToolUses && (
        <div className="flex flex-col gap-1">
          {message.tool_uses.map((tool, index) => (
            <ToolUseChip
              key={tool.tool_id}
              tool={tool}
              blockId={`message-${blockIndex}-tool-${index}`}
            />
          ))}
        </div>
      )}
    </div>
  );
}

// Thinking Block Component
function ThinkingBlock({ content, blockId }: { content: string; blockId: string }) {
  const [isExpanded, setIsExpanded] = useState(false);

  return (
    <div id={blockId} className="thinking-block">
      <button
        type="button"
        className="thinking-trigger"
        onClick={() => setIsExpanded(!isExpanded)}
        aria-expanded={isExpanded}
      >
        <ChevronRight className="h-4 w-4" />
        <Brain className="h-3.5 w-3.5 mr-0.5" />
        <span>Thinking</span>
      </button>

      <div className={cn('thinking-content', isExpanded && 'expanded')}>
        <div className="markdown-content">
          <MarkdownRenderer content={content} />
        </div>
      </div>
    </div>
  );
}

// Tool Use Chip Component
function ToolUseChip({ tool, blockId }: { tool: ToolUse; blockId: string }) {
  const [isExpanded, setIsExpanded] = useState(false);
  const ToolIcon = getToolIcon(tool.tool_name);
  const summary = getToolSummary(tool);
  const isFileOperation = ['Read', 'Write', 'Edit'].includes(tool.tool_name);

  // Compact chip for simple tools
  if (!isFileOperation && !isExpanded) {
    return (
      <div id={blockId} className="resource-chip">
        <button
          type="button"
          className="resource-chip-compact"
          onClick={() => setIsExpanded(true)}
        >
          <div className="chip-inner">
            <ToolIcon className="h-4 w-4" />
            <div className="chip-label">
              <span>{summary}</span>
              {tool.output && (
                <span className={cn(
                  'tool-status ml-auto',
                  tool.output.success ? 'tool-status-success' : 'tool-status-error'
                )}>
                  {tool.output.success ? '✓' : '✗'}
                </span>
              )}
            </div>
          </div>
        </button>
      </div>
    );
  }

  // Expanded chip for file operations
  const filePath = (tool.input.file_path || tool.input.path) as string | undefined;

  return (
    <div id={blockId} className="resource-chip">
      <button
        type="button"
        className="resource-chip-expanded"
        onClick={() => setIsExpanded(!isExpanded)}
      >
        <div className="chip-inner">
          <ToolIcon className="h-4 w-4" />
          <div className="file-path">
            {filePath ? (
              <>
                <span className="path-prefix">
                  {filePath.split('/').slice(0, -1).join('/')}
                </span>
                <span className="path-separator">/</span>
                <span className="path-basename">
                  {filePath.split('/').pop()}
                </span>
              </>
            ) : (
              <span>{summary}</span>
            )}
          </div>
          {tool.output && (
            <span className={cn(
              'tool-status',
              tool.output.success ? 'tool-status-success' : 'tool-status-error'
            )}>
              {tool.output.success ? 'Success' : 'Failed'}
            </span>
          )}
        </div>
      </button>

      {/* Expanded content */}
      {isExpanded && (
        <div className="border-t border-border/50 p-3">
          <div className="text-xs text-muted-foreground mb-2">Input</div>
          <CodeBlock
            code={JSON.stringify(tool.input, null, 2)}
            language="json"
          />

          {tool.output && (
            <>
              <div className="text-xs text-muted-foreground mb-2 mt-3">Output</div>
              {tool.output.error ? (
                <div className="rounded bg-red-50 dark:bg-red-900/20 p-3 text-sm text-red-800 dark:text-red-300">
                  {tool.output.error}
                </div>
              ) : (
                <CodeBlock
                  code={
                    typeof tool.output.content === 'string'
                      ? tool.output.content.slice(0, 2000) + (tool.output.content.length > 2000 ? '\n... truncated' : '')
                      : JSON.stringify(tool.output.content, null, 2)
                  }
                  language="json"
                />
              )}
            </>
          )}
        </div>
      )}
    </div>
  );
}

// Simple Markdown Renderer
function MarkdownRenderer({ content }: { content: string }) {
  const elements = useMemo(() => parseMarkdown(content), [content]);
  return <>{elements}</>;
}

function parseMarkdown(content: string): React.ReactNode[] {
  const elements: React.ReactNode[] = [];
  const lines = content.split('\n');
  let inCodeBlock = false;
  let codeBlockLang = '';
  let codeBlockContent: string[] = [];
  let key = 0;

  for (const line of lines) {
    if (line.startsWith('```')) {
      if (inCodeBlock) {
        // End code block
        elements.push(
          <CodeBlock
            key={key++}
            code={codeBlockContent.join('\n')}
            language={codeBlockLang}
          />
        );
        inCodeBlock = false;
        codeBlockContent = [];
      } else {
        // Start code block
        inCodeBlock = true;
        codeBlockLang = line.slice(3).trim();
      }
      continue;
    }

    if (inCodeBlock) {
      codeBlockContent.push(line);
      continue;
    }

    // Headers
    if (line.startsWith('### ')) {
      elements.push(<h3 key={key++}>{line.slice(4)}</h3>);
    } else if (line.startsWith('## ')) {
      elements.push(<h2 key={key++}>{line.slice(3)}</h2>);
    } else if (line.startsWith('# ')) {
      elements.push(<h1 key={key++}>{line.slice(2)}</h1>);
    } else if (line.startsWith('- ') || line.startsWith('* ')) {
      elements.push(
        <ul key={key++}>
          <li>{renderInlineMarkdown(line.slice(2))}</li>
        </ul>
      );
    } else if (line.trim() === '') {
      // Skip empty lines but add spacing
    } else {
      elements.push(<p key={key++}>{renderInlineMarkdown(line)}</p>);
    }
  }

  return elements;
}

function renderInlineMarkdown(text: string): React.ReactNode {
  // Handle inline code
  const parts = text.split(/(`[^`]+`)/g);
  return parts.map((part, index) => {
    if (part.startsWith('`') && part.endsWith('`')) {
      return <code key={index}>{part.slice(1, -1)}</code>;
    }
    // Handle bold
    if (part.includes('**')) {
      const boldParts = part.split(/(\*\*[^*]+\*\*)/g);
      return boldParts.map((bp, i) => {
        if (bp.startsWith('**') && bp.endsWith('**')) {
          return <strong key={`${index}-${i}`}>{bp.slice(2, -2)}</strong>;
        }
        return bp;
      });
    }
    return part;
  });
}

// Code Block Component
function CodeBlock({ code, language }: { code: string; language?: string }) {
  const [copied, setCopied] = useState(false);

  const handleCopy = async () => {
    await navigator.clipboard.writeText(code);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="relative rounded-lg bg-zinc-950 dark:bg-zinc-900 overflow-hidden">
      {language && (
        <div className="flex items-center justify-between border-b border-zinc-800 px-4 py-2">
          <span className="text-xs text-zinc-400">{language}</span>
          <Button
            variant="ghost"
            size="icon"
            className="h-6 w-6 text-zinc-400 hover:text-zinc-200"
            onClick={handleCopy}
          >
            {copied ? <Check className="h-3 w-3" /> : <Copy className="h-3 w-3" />}
          </Button>
        </div>
      )}
      <pre className="overflow-x-auto p-4">
        <code className="text-sm text-zinc-100">{code}</code>
      </pre>
    </div>
  );
}

// Session Stats Component
function SessionStats({ session }: { session: Session }) {
  const toolUsage = useMemo(() => {
    const usage: Record<string, number> = {};
    for (const msg of session.messages) {
      for (const tool of msg.tool_uses) {
        usage[tool.tool_name] = (usage[tool.tool_name] || 0) + 1;
      }
    }
    return Object.entries(usage).sort(([, a], [, b]) => b - a);
  }, [session.messages]);

  const duration = session.metadata.duration_seconds
    ? formatDuration(session.metadata.duration_seconds)
    : null;

  return (
    <div className="mt-8 rounded-lg border bg-card p-6">
      <h2 className="font-semibold mb-4">Session Statistics</h2>

      <div className="grid grid-cols-2 sm:grid-cols-4 gap-4 mb-6">
        <StatCard
          icon={MessageSquare}
          label="Messages"
          value={session.metadata.message_count}
        />
        <StatCard
          icon={Wrench}
          label="Tool Calls"
          value={session.metadata.tool_call_count}
        />
        {duration && (
          <StatCard icon={Clock} label="Duration" value={duration} />
        )}
        {session.metadata.cwd && (
          <StatCard
            icon={Folder}
            label="Directory"
            value={session.metadata.cwd.split('/').pop() || ''}
          />
        )}
      </div>

      {toolUsage.length > 0 && (
        <div>
          <h3 className="text-sm font-medium text-muted-foreground mb-3">
            Tool Usage
          </h3>
          <div className="flex flex-wrap gap-2">
            {toolUsage.map(([tool, count]) => (
              <div
                key={tool}
                className="flex items-center gap-2 rounded-full bg-muted px-3 py-1"
              >
                <span className="text-sm font-medium">{tool}</span>
                <span className="rounded-full bg-background px-2 py-0.5 text-xs">
                  {count}
                </span>
              </div>
            ))}
          </div>
        </div>
      )}
    </div>
  );
}

// Stat Card Component
function StatCard({
  icon: Icon,
  label,
  value
}: {
  icon: React.ComponentType<{ className?: string }>;
  label: string;
  value: number | string;
}) {
  return (
    <div className="rounded-lg bg-muted/50 p-4">
      <div className="flex items-center gap-2 text-muted-foreground mb-1">
        <Icon className="h-4 w-4" />
        <span className="text-xs">{label}</span>
      </div>
      <p className="text-xl font-bold">{value}</p>
    </div>
  );
}

// Helper Functions
function formatDuration(seconds: number): string {
  if (seconds < 60) return `${seconds}s`;
  if (seconds < 3600) return `${Math.floor(seconds / 60)}m ${seconds % 60}s`;
  const hours = Math.floor(seconds / 3600);
  const minutes = Math.floor((seconds % 3600) / 60);
  return `${hours}h ${minutes}m`;
}

function extractFileReferences(text: string): string[] {
  // Match @filepath or file:// references
  const matches = text.match(/@[\w./\-]+|file:\/\/[^\s]+/g) || [];
  return matches.map(m => m.replace(/^@|^file:\/\//, ''));
}

function getToolIcon(toolName: string): React.ComponentType<{ className?: string }> {
  const icons: Record<string, React.ComponentType<{ className?: string }>> = {
    Read: FileText,
    Write: FileText,
    Edit: FileText,
    Bash: Terminal,
    Grep: Search,
    Glob: Files,
    Task: Bot,
    TodoWrite: ListTodo,
    TodoRead: ListTodo,
  };
  return icons[toolName] || Wrench;
}

function getToolSummary(tool: ToolUse): string {
  const input = tool.input;
  switch (tool.tool_name) {
    case 'Read':
      return `Read ${(input.file_path as string)?.split('/').pop() || 'file'}`;
    case 'Write':
      return `Write ${(input.file_path as string)?.split('/').pop() || 'file'}`;
    case 'Edit':
      return `Edit ${(input.file_path as string)?.split('/').pop() || 'file'}`;
    case 'Bash':
      return (input.command as string)?.slice(0, 50) || 'Running command...';
    case 'Grep':
      return `Search "${input.pattern}" in ${input.path || '.'}`;
    case 'Glob':
      return `Find ${input.pattern as string || 'files'}`;
    case 'Task':
      return input.description as string || 'Running task...';
    case 'TodoWrite':
      return 'Updated TODOs';
    case 'TodoRead':
      return 'Read TODOs';
    default:
      return tool.tool_name;
  }
}

export default SessionViewer;

// Export types for external use
export type { Session, Message, ToolUse, SessionMetadata };
