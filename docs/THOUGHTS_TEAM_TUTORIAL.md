# Team Thoughts Tutorial

*Follow Team Nexus through their first week using the thoughts system. By the end, you'll have hands-on experience with every core command.*

**Prerequisites**: A project directory with git initialized.

**Time**: ~30 minutes to complete all exercises.

---

## Day 1: Getting Started

### Story

Sarah, the tech lead, gathers her team: Marcus (backend), Priya (frontend), and Alex (junior developer).

> **Sarah**: "We're starting a shared knowledge system today. Let me show you."

### Exercise 1: Initialize Thoughts

Open your terminal in your project directory and run:

```bash
snps thoughts init
```

You should see:
```
✓ Created thoughts symlink
✓ Created shared/ directory
✓ Created [your-username]/ personal directory
✓ Created global/ symlink
✓ Installed pre-commit hook
```

Verify the structure:

```bash
ls thoughts/
```

You should see: `shared/`, `[your-username]/`, `global/`, and possibly `searchable/`.

---

### Story

Marcus receives ticket **ENG-234: Add webhook support**. He doesn't know where the notification code lives.

> **Marcus**: "I have no idea how our notification system works."
>
> **Sarah**: "Document what you learn. Your future self will thank you."

### Exercise 2: Create a Research Document

Create your first research document:

```bash
snps thoughts new research "Learning the Codebase"
```

The command creates a file and opens it. Fill in the template:

```markdown
---
date: 2025-12-13
author: [your-username]
type: research
status: in_progress
tags: [learning]
---

# Research: Learning the Codebase

## Question
What is the overall structure of this project?

## Key Findings

### Finding 1: Project Structure
[Look at the project and write what you see]

## Relevant Files
- `src/` - [What's in here?]

## Open Questions
- [ ] [Something you're still unsure about]
```

Save the file.

---

### Exercise 3: Sync Your Work

Save your research to the central repository:

```bash
snps thoughts sync -m "Initial codebase exploration"
```

You should see:
```
Syncing thoughts...
  ✓ Rebuilt searchable index (X files)
  ✓ Committed: Initial codebase exploration
✅ Sync complete
```

---

## Day 2: From Research to Plan

### Story

Sarah reviews Marcus's research the next morning.

> **Sarah**: "Good research. Now let's make a plan before you write code."
>
> **Marcus**: "Why? I know what to do."
>
> **Sarah**: "Plans let me catch problems before you've written 500 lines."

### Exercise 4: Create a Plan

Create a plan document:

```bash
snps thoughts new plan "Add New Feature"
```

Fill in the template with phases and checkboxes:

```markdown
---
date: 2025-12-13
author: [your-username]
type: plan
status: pending_approval
tags: [feature]
---

# Plan: Add New Feature

## Overview
[One sentence: what are you building?]

## Research Reference
Based on: [[research/2025-12-13-learning-the-codebase.md]]

## Implementation Phases

### Phase 1: Setup
- [ ] Create new file
- [ ] Add basic structure

### Phase 2: Implementation
- [ ] Implement core logic
- [ ] Add error handling

### Phase 3: Testing
- [ ] Write unit tests
- [ ] Manual verification

## Success Criteria
- [ ] All tests pass
- [ ] Feature works as expected
```

Save and sync:

```bash
snps thoughts sync -m "Created feature plan"
```

---

### Exercise 5: Search Existing Thoughts

Before starting new work, check what already exists:

```bash
snps thoughts search "codebase"
```

You should see your research document in the results.

Try listing recent documents:

```bash
snps thoughts list --recent 5
```

---

## Day 3: Track Progress

### Story

Marcus starts implementing. As he completes each item, he checks it off in the plan.

> **Sarah**: "Update the checkboxes as you go. If you get sick, someone can pick up where you left off."

### Exercise 6: Update Plan Progress

Open your plan file and change checkboxes from `- [ ]` to `- [x]` as you complete items:

```markdown
### Phase 1: Setup
- [x] Create new file
- [x] Add basic structure
```

Sync your progress:

```bash
snps thoughts sync -m "Phase 1 complete"
```

---

## Day 4: Personal Notes

### Story

Alex, the junior developer, feels overwhelmed by their first ticket. Sarah suggests personal notes.

> **Sarah**: "Write notes as you learn. They're just for you—no pressure to be polished."

### Exercise 7: Create Personal Notes

Create a personal scratch document:

```bash
snps thoughts new scratch "My Learning Notes" --scope personal
```

Write anything helpful:

```markdown
# My Learning Notes

## Things I Learned Today
- The database is PostgreSQL
- Tests run with `cargo test`

## Questions to Ask
- What's the deployment process?

## Mistakes I Made (so I don't repeat them)
- Forgot to run migrations before testing
```

Sync (personal notes sync too):

```bash
snps thoughts sync -m "Personal notes"
```

---

### Exercise 8: Check System Status

See your thoughts configuration:

```bash
snps thoughts status
```

For more detail:

```bash
snps thoughts status --verbose
```

---

## Day 5: Handoffs

### Story

Marcus is out sick. Priya needs to finish his work.

> **Priya**: "I've never touched that code..."
>
> **Sarah**: "Check Marcus's thoughts. He documented everything."

### Exercise 9: Find Someone Else's Work

Search for a specific topic:

```bash
snps thoughts search "feature" --doc-type plan
```

Open the plan and see the checkbox progress. You can immediately tell:
- What's done (checked boxes)
- What's remaining (unchecked boxes)
- The overall approach (the plan text)

---

## Week 2: Patterns

### Story

Sarah notices the team solving the same problems repeatedly. She creates a shared pattern.

> **Sarah**: "When you solve something others might need, add it to global patterns."

### Exercise 10: Create a Global Pattern

Create a pattern in the global scope:

```bash
snps thoughts new research "Error Handling Pattern" --scope global
```

Document the pattern:

```markdown
# Pattern: Error Handling

## When to Use
Any new API endpoint.

## The Pattern
[Describe the standard approach]

## Example
[Show a code example from the codebase]
```

Sync:

```bash
snps thoughts sync -m "Added error handling pattern"
```

---

## Completion Checklist

You've completed the tutorial if you can check all these boxes:

- [ ] Initialized thoughts in a project (`snps thoughts init`)
- [ ] Created a research document (`snps thoughts new research`)
- [ ] Created a plan with checkboxes (`snps thoughts new plan`)
- [ ] Searched existing thoughts (`snps thoughts search`)
- [ ] Listed recent documents (`snps thoughts list`)
- [ ] Updated plan checkboxes and synced
- [ ] Created personal notes (`--scope personal`)
- [ ] Checked system status (`snps thoughts status`)
- [ ] Created a global pattern (`--scope global`)

---

## Next Steps

Now that you've learned the basics through practice:

- **For command details**: See [Thoughts Reference](./THOUGHTS_REFERENCE.md)
- **For workflow guidance**: See [Thoughts Workflow Tutorial](./THOUGHTS_WORKFLOW_TUTORIAL.md)
- **For system architecture**: See [Thoughts System](./THOUGHTS_SYSTEM.md)

---

*Part of PMSynapse AI-Enabled Knowledge Management*
