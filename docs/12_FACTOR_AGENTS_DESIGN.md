# 12-Factor Agents: Agentic System Design Principles

## Overview

This document captures the **12-Factor Agents** methodology from HumanLayer for building production-ready agentic AI systems. These principles should guide the design of PMSynapse from day one to ensure reliability, maintainability, and scalability.

**Source**: [HumanLayer 12-Factor Agents](https://github.com/humanlayer/12-factor-agents)
**Author**: Dex Horthy (HumanLayer)

---

## The Core Insight

> "Most of the products billing themselves as 'AI Agents' are not all that agentic. A lot of them are mostly deterministic code, with LLM steps sprinkled in at just the right points to make the experience truly magical."

**The 70-80% Problem**: Pure agent loops hit a reliability ceiling around 70-80%. They're prone to:
- Looping endlessly
- Making malformed tool calls
- Losing track of state
- Context window degradation

**The Solution**: Build systems that are **mostly deterministic code** with **strategic LLM decision points**.

---

## The 12 Factors

### Factor 1: Natural Language to Tool Calls

**Principle**: The core job of an LLM in an agent is converting natural language into structured JSON that triggers deterministic actions.

```
User: "Create a $750 payment link for the enterprise plan"
              │
              ▼
         ┌─────────┐
         │   LLM   │  ← Structured output extraction
         └────┬────┘
              │
              ▼
{
  "intent": "create_payment_link",
  "amount": 750,
  "plan": "enterprise"
}
              │
              ▼
    Deterministic Code
    (Stripe API call)
```

**Best Practice for PMSynapse**:
- Define clear tool schemas for all agent operations
- LLM decides WHAT to do, deterministic code decides HOW
- Never let LLM directly execute side effects

---

### Factor 2: Own Your Prompts

**Principle**: Treat prompts as first-class code. Avoid framework abstractions that hide prompt construction.

**Anti-Pattern**:
```python
# Don't do this - you don't control what's sent to the LLM
agent = SomeFramework.create_agent(tools=[...])
agent.run("do something")
```

**Best Practice**:
```python
# Do this - full control over every token
prompt = f"""
You are a task decomposition agent for PMSynapse.

## Your Role
Break down the following PRD into actionable tasks.

## Rules
- Each task must be completable in under 4 hours
- Include dependencies between tasks
- Score complexity 1-10

## PRD Content
{prd_content}

## Output Format
Return valid JSON matching this schema:
{json.dumps(task_schema)}
"""

response = llm.complete(prompt)
```

**Best Practice for PMSynapse**:
- Store prompts in version-controlled files
- Test prompts like code (input → expected output)
- Log full prompts for debugging
- Iterate rapidly without framework constraints

---

### Factor 3: Own Your Context Window

**Principle**: Carefully curate what goes into the context window. Don't blindly append messages.

**The "Dumb Zone"**: Analysis of 100,000 sessions revealed that the middle 40-60% of a context window has degraded recall and reasoning. Fill past 40% and diminishing returns begin.

```
┌─────────────────────────────────────────────────────────┐
│                  CONTEXT WINDOW                          │
├─────────────────────────────────────────────────────────┤
│ 0-20%     │ Strong recall, good reasoning               │
│ 20-40%    │ Good performance                            │
│ 40-60%    │ ⚠️ "DUMB ZONE" - degraded performance       │
│ 60-80%    │ Significantly degraded                      │
│ 80-100%   │ Poor performance, hallucinations            │
└─────────────────────────────────────────────────────────┘
```

**Best Practice for PMSynapse**:
- Target 40-60% context utilization maximum
- Use custom formatting (XML, YAML) over chat format
- Summarize history instead of including full transcripts
- Use RAG to pull in relevant context on-demand

```python
# Custom context formatting
context = f"""
<system>
You are a PMSynapse coordination agent.
</system>

<current_task>
{task.to_xml()}
</current_task>

<relevant_decisions>
{format_as_xml(recent_decisions[:5])}
</relevant_decisions>

<available_tools>
{format_tools_yaml(tools)}
</available_tools>
"""
```

---

### Factor 4: Tools Are Just Structured Outputs

**Principle**: Tools are simply JSON outputs that trigger deterministic code paths. Demystify them.

```python
# Tool definitions are just output schemas
class CreateTask(BaseModel):
    intent: Literal["create_task"] = "create_task"
    title: str
    description: str
    complexity: int  # 1-10
    dependencies: list[str]

class SearchThoughts(BaseModel):
    intent: Literal["search_thoughts"] = "search_thoughts"
    query: str
    scope: Literal["shared", "personal", "global", "all"]

class ContactHuman(BaseModel):
    intent: Literal["contact_human"] = "contact_human"
    message: str
    channel: Literal["slack", "email", "in_app"]
    urgency: Literal["low", "medium", "high"]

# Execution is deterministic routing
def execute_tool(tool_output: dict):
    match tool_output["intent"]:
        case "create_task":
            return task_service.create(tool_output)
        case "search_thoughts":
            return thoughts_service.search(tool_output)
        case "contact_human":
            return human_layer.contact(tool_output)
```

**Best Practice for PMSynapse**:
- Define all tools as Pydantic/dataclass schemas
- Routing logic is simple match/switch statements
- Tool execution is 100% deterministic
- LLM never directly calls external APIs

---

### Factor 5: Unify Execution State and Business State

**Principle**: Use a single event log for all state. Don't maintain separate databases for "what happened" vs "what's the current status".

**Anti-Pattern**:
```
execution_db: {step: 3, status: "running", retries: 2}
business_db: {task_id: 123, completed: false, assignee: "alice"}
```

**Best Practice**:
```python
# Single event log captures everything
events = [
    {"type": "task_created", "task_id": 123, "title": "Implement auth"},
    {"type": "agent_started", "agent_id": "decomposer", "timestamp": "..."},
    {"type": "tool_called", "tool": "search_thoughts", "query": "auth patterns"},
    {"type": "tool_result", "tool": "search_thoughts", "results": [...]},
    {"type": "decision_made", "decision": "use_jwt", "reasoning": "..."},
    {"type": "human_contacted", "channel": "slack", "message": "..."},
    {"type": "human_responded", "response": "approved", "timestamp": "..."},
    {"type": "task_completed", "task_id": 123, "timestamp": "..."}
]

# Current state is derived from events
def get_current_state(events):
    state = initial_state()
    for event in events:
        state = apply_event(state, event)
    return state
```

**Best Practice for PMSynapse**:
- Every action is an event in the log
- State is always derivable from events
- Enables perfect debugging and replay
- Supports forking agent execution for A/B testing

---

### Factor 6: Launch/Pause/Resume with Simple APIs

**Principle**: Agents must be able to pause (awaiting human input, external API, long job) and resume cleanly.

```python
class AgentExecution:
    id: str
    status: Literal["running", "paused", "completed", "failed"]
    pause_reason: Optional[str]
    resume_webhook: Optional[str]
    events: list[Event]

# Pause when waiting for human
def handle_contact_human(execution, tool_call):
    execution.status = "paused"
    execution.pause_reason = "awaiting_human_response"
    execution.resume_webhook = f"/api/executions/{execution.id}/resume"

    # Send to human
    human_layer.send(
        message=tool_call.message,
        callback_url=execution.resume_webhook
    )

    return execution

# Resume via webhook
@app.post("/api/executions/{id}/resume")
def resume_execution(id: str, human_response: str):
    execution = load_execution(id)
    execution.events.append({
        "type": "human_responded",
        "response": human_response
    })
    execution.status = "running"

    # Continue agent loop
    return continue_agent(execution)
```

**Best Practice for PMSynapse**:
- Every execution has a unique ID
- Pause states are explicit and logged
- Resume is idempotent (can retry safely)
- Webhooks enable async human-in-the-loop

---

### Factor 7: Contact Humans with Tool Calls

**Principle**: Human interaction is just another tool. Structure it, don't leave it as freeform text.

```python
class ContactHuman(BaseModel):
    """Request human input or approval"""
    intent: Literal["contact_human"] = "contact_human"
    request_type: Literal["approval", "input", "clarification", "escalation"]
    message: str
    context: str  # What the human needs to know
    options: Optional[list[str]]  # For approval/choice requests
    channel: Literal["slack", "email", "in_app"]
    urgency: Literal["low", "medium", "high", "critical"]
    timeout_hours: int = 24

# Example outputs from LLM
{
    "intent": "contact_human",
    "request_type": "approval",
    "message": "Ready to deploy auth service to production",
    "context": "All tests passing. 3 files changed. No breaking changes detected.",
    "options": ["approve", "reject", "defer"],
    "channel": "slack",
    "urgency": "medium"
}
```

**Best Practice for PMSynapse**:
- Define clear request types (approval, input, clarification, escalation)
- Include context for human decision-making
- Support multiple channels from day one
- Track response times and outcomes

---

### Factor 8: Own Your Control Flow

**Principle**: Don't rely on framework "auto-retry" or "auto-loop". Implement explicit control flow.

```python
def agent_loop(execution: AgentExecution):
    while True:
        # Get next action from LLM
        action = get_next_action(execution)

        match action.intent:
            case "create_task" | "search_thoughts" | "update_status":
                # Immediate execution, continue loop
                result = execute_tool(action)
                execution.events.append({"type": "tool_result", "result": result})

            case "contact_human":
                # Pause and wait for human
                pause_for_human(execution, action)
                return  # Exit loop, will resume via webhook

            case "wait_for_background_job":
                # Pause and wait for job completion
                pause_for_job(execution, action)
                return  # Exit loop, will resume via webhook

            case "complete":
                # Done!
                execution.status = "completed"
                return

            case "error":
                # Handle error with retry logic
                if execution.error_count < 3:
                    execution.error_count += 1
                    execution.events.append({"type": "error", "error": action.error})
                    # Continue loop - LLM will see error in context
                else:
                    # Escalate to human
                    escalate_to_human(execution, action.error)
                    return
```

**Best Practice for PMSynapse**:
- Explicit handling for each action type
- Different actions have different flow implications
- Error handling is deliberate, not automatic
- Loop exits are explicit and logged

---

### Factor 9: Compact Errors into Context Window

**Principle**: When errors occur, include them in context so the LLM can attempt recovery.

```python
def format_error_for_context(error: Exception, attempt: int) -> str:
    return f"""
<error attempt="{attempt}" max_attempts="3">
  <type>{type(error).__name__}</type>
  <message>{str(error)}</message>
  <suggestion>
    {get_recovery_suggestion(error)}
  </suggestion>
</error>
"""

# In agent loop
try:
    result = execute_tool(action)
except Exception as e:
    if execution.error_count < 3:
        error_context = format_error_for_context(e, execution.error_count)
        execution.events.append({
            "type": "error",
            "error_context": error_context
        })
        # LLM will see error and can adjust approach
    else:
        escalate_to_human(execution, e)
```

**Best Practice for PMSynapse**:
- Include error type, message, and recovery suggestions
- Limit retry attempts (typically 3)
- Escalate to humans after max retries
- Log all errors for debugging

---

### Factor 10: Small, Focused Agents

**Principle**: Build agents that handle 3-10 steps, not 100+ step monoliths.

```
❌ ANTI-PATTERN: Monolithic Agent
┌─────────────────────────────────────────────────────────┐
│  "Universal Agent"                                       │
│  - Parse PRD                                            │
│  - Decompose into tasks                                 │
│  - Assign to developers                                 │
│  - Implement each task                                  │
│  - Write tests                                          │
│  - Review code                                          │
│  - Deploy to staging                                    │
│  - Run integration tests                                │
│  - Deploy to production                                 │
│  - Monitor for errors                                   │
│  - ... 50 more steps                                    │
│                                                          │
│  RESULT: Context bloat, lost focus, 70% reliability     │
└─────────────────────────────────────────────────────────┘

✅ BEST PRACTICE: Focused Micro-Agents
┌──────────────┐  ┌──────────────┐  ┌──────────────┐
│  PRD Parser  │  │    Task      │  │  Conflict    │
│    Agent     │  │  Decomposer  │  │  Detector    │
│  (3 steps)   │  │  (5 steps)   │  │  (4 steps)   │
└──────────────┘  └──────────────┘  └──────────────┘

┌──────────────┐  ┌──────────────┐  ┌──────────────┐
│   Research   │  │    Plan      │  │   Thoughts   │
│    Agent     │  │   Writer     │  │   Locator    │
│  (4 steps)   │  │  (5 steps)   │  │  (3 steps)   │
└──────────────┘  └──────────────┘  └──────────────┘

RESULT: Each agent is focused, testable, reliable (95%+)
```

**Best Practice for PMSynapse**:
- Each agent has ONE clear responsibility
- 3-10 steps maximum per agent
- Agents can call other agents (composition)
- Scale agent scope only as models improve

---

### Factor 11: Trigger from Anywhere

**Principle**: Agents should be invokable from multiple sources and respond through multiple channels.

```python
# Triggers (inputs)
triggers = [
    "webhook",      # External systems
    "cron",         # Scheduled
    "user_message", # Direct interaction
    "event",        # Internal events (task completed, PR merged)
    "human_response", # Resuming paused execution
]

# Channels (outputs)
channels = [
    "slack",
    "email",
    "in_app",
    "github_comment",
    "webhook_callback",
]

# Agent is channel-agnostic
class AgentExecution:
    trigger_source: str
    trigger_data: dict
    response_channel: str
    response_callback: Optional[str]
```

**Best Practice for PMSynapse**:
- Design agents to be trigger-agnostic
- Support multiple response channels from day one
- Background agents work until needing human decision
- All channels use the same underlying agent logic

---

### Factor 12: Make Your Agent a Stateless Reducer

**Principle**: Agent execution should be a pure function: `(previous_state, new_event) → next_state`

```python
# Agent as pure function
def agent_step(state: AgentState, event: Event) -> AgentState:
    """
    Pure function: given current state and new event,
    return the next state.

    No side effects inside this function.
    """
    new_state = state.copy()

    match event.type:
        case "user_input":
            new_state.context.append(event.input)
            new_state.pending_action = determine_action(new_state)

        case "tool_result":
            new_state.context.append(format_result(event.result))
            new_state.pending_action = determine_action(new_state)

        case "human_response":
            new_state.context.append(format_human_response(event.response))
            new_state.status = "running"
            new_state.pending_action = determine_action(new_state)

        case "error":
            new_state.error_count += 1
            new_state.context.append(format_error(event.error))
            new_state.pending_action = determine_action(new_state)

    return new_state

# Execution is separate from state computation
def execute_agent(execution_id: str):
    state = load_state(execution_id)

    while state.status == "running":
        # Execute pending action (side effect)
        result = execute_action(state.pending_action)

        # Compute next state (pure)
        event = Event(type="tool_result", result=result)
        state = agent_step(state, event)

        # Persist state
        save_state(execution_id, state)

    return state
```

**Benefits**:
- **Reproducibility**: Replay any execution from event log
- **Debugging**: Inspect state at any point in time
- **Forking**: Create branches from any state
- **Testing**: Pure functions are easy to test
- **Scaling**: Stateless enables horizontal scaling

---

## Application to PMSynapse

### Agent Architecture

```
┌─────────────────────────────────────────────────────────┐
│                  PMSYNAPSE AGENTS                        │
├─────────────────────────────────────────────────────────┤
│                                                          │
│  COORDINATION LAYER (Deterministic)                     │
│  ├── Event Router                                       │
│  ├── Execution Manager (launch/pause/resume)            │
│  ├── State Store (event log)                            │
│  └── Human Contact Service                              │
│                                                          │
│  MICRO-AGENTS (LLM-powered, 3-10 steps each)            │
│  ├── PRD Parser Agent                                   │
│  │   └── Natural language → structured requirements     │
│  │                                                       │
│  ├── Task Decomposer Agent                              │
│  │   └── Requirements → dependency-aware tasks          │
│  │                                                       │
│  ├── Complexity Scorer Agent                            │
│  │   └── Task → complexity score + expansion decision   │
│  │                                                       │
│  ├── Conflict Detector Agent                            │
│  │   └── Intent → potential conflicts + recommendations │
│  │                                                       │
│  ├── Thoughts Locator Agent                             │
│  │   └── Query → relevant document paths                │
│  │                                                       │
│  ├── Research Synthesizer Agent                         │
│  │   └── Documents → structured research summary        │
│  │                                                       │
│  ├── Plan Writer Agent                                  │
│  │   └── Research + task → implementation plan          │
│  │                                                       │
│  └── Progress Reporter Agent                            │
│      └── Events → human-readable status update          │
│                                                          │
│  TOOL EXECUTION (Deterministic)                         │
│  ├── Task CRUD operations                               │
│  ├── Thoughts search/read/write                         │
│  ├── Knowledge base queries                             │
│  ├── External API calls                                 │
│  └── Human contact dispatch                             │
│                                                          │
└─────────────────────────────────────────────────────────┘
```

### Implementation Checklist

#### Foundation (Week 1)
- [ ] Define tool schemas (Pydantic models)
- [ ] Implement event log storage
- [ ] Build deterministic tool execution layer
- [ ] Create prompt templates (version controlled)

#### Control Flow (Week 2)
- [ ] Implement agent loop with explicit control flow
- [ ] Add launch/pause/resume APIs
- [ ] Build webhook handlers for resumption
- [ ] Add error handling with retry limits

#### Human-in-the-Loop (Week 3)
- [ ] Implement ContactHuman tool
- [ ] Build Slack integration
- [ ] Add email fallback
- [ ] Create in-app notification system

#### Micro-Agents (Week 4+)
- [ ] PRD Parser Agent
- [ ] Task Decomposer Agent
- [ ] Thoughts Locator Agent
- [ ] Add more agents incrementally

---

## Key Metrics to Track

| Metric | Target | Why |
|--------|--------|-----|
| Agent success rate | >95% | Reliability indicator |
| Steps per execution | <10 | Complexity indicator |
| Context utilization | 40-60% | Performance sweet spot |
| Human escalation rate | <20% | Automation effectiveness |
| Time to human response | <1 hour | Workflow velocity |
| Error recovery rate | >80% | Resilience indicator |

---

## Anti-Patterns to Avoid

### 1. Framework Lock-in
```python
# ❌ Don't do this
from magic_framework import AutoAgent
agent = AutoAgent(tools=[...])  # Black box
agent.run()

# ✅ Do this
prompt = build_prompt(context, tools)
response = llm.complete(prompt)
action = parse_tool_call(response)
result = execute_tool(action)
```

### 2. Unbounded Loops
```python
# ❌ Don't do this
while True:
    response = llm.complete(context)
    # Hope it eventually says "done"

# ✅ Do this
MAX_STEPS = 10
for step in range(MAX_STEPS):
    action = get_next_action(state)
    if action.intent == "complete":
        break
    if action.intent == "contact_human":
        pause_and_wait()
        break
    execute_and_continue(action)
else:
    escalate_to_human("Max steps reached")
```

### 3. Implicit State
```python
# ❌ Don't do this
class Agent:
    def __init__(self):
        self.messages = []  # Mutable state
        self.step = 0

# ✅ Do this
@dataclass
class AgentState:
    events: list[Event]
    status: str

def next_state(state: AgentState, event: Event) -> AgentState:
    # Pure function, returns new state
    return AgentState(
        events=[*state.events, event],
        status=compute_status(state, event)
    )
```

### 4. Monolithic Agents
```python
# ❌ Don't do this
class UniversalAgent:
    def handle_everything(self, request):
        # 50+ different responsibilities
        pass

# ✅ Do this
class TaskDecomposer:  # 5 steps max
    pass

class ConflictDetector:  # 4 steps max
    pass

class ThoughtsLocator:  # 3 steps max
    pass
```

---

## Summary: The 12 Factors at a Glance

| # | Factor | Key Principle |
|---|--------|---------------|
| 1 | Natural Language → Tool Calls | LLM extracts structured JSON, code executes |
| 2 | Own Your Prompts | Prompts are code, version control them |
| 3 | Own Your Context Window | Curate context, target 40-60% utilization |
| 4 | Tools = Structured Outputs | Tools are just JSON → deterministic routing |
| 5 | Unified State | Single event log for all state |
| 6 | Launch/Pause/Resume | Explicit lifecycle management |
| 7 | Humans as Tool Calls | Structured human interaction |
| 8 | Own Control Flow | Explicit loop logic, not framework magic |
| 9 | Compact Errors | Include errors in context for recovery |
| 10 | Small Focused Agents | 3-10 steps, single responsibility |
| 11 | Trigger Anywhere | Multi-channel input and output |
| 12 | Stateless Reducer | Pure function: (state, event) → state |

---

## Sources

- [HumanLayer 12-Factor Agents](https://github.com/humanlayer/12-factor-agents)
- [HumanLayer Blog - 12 Factor Agents](https://www.humanlayer.dev/blog/12-factor-agents)
- [DEV.to - 12-Factor Agent Framework](https://dev.to/bredmond1019/the-12-factor-agent-a-practical-framework-for-building-production-ai-systems-3oo8)
- [IK Insights - 12-Factor Agents Blueprint](https://www.ikangai.com/12-factor-agents-a-blueprint-for-reliable-llm-applications/)

---

*Document created: December 2025*
*Part of: PMSynapse AI-Enabled Knowledge Management Research*
