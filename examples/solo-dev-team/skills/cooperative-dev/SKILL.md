---
name: "cooperative-dev"
description: "Coordinate Architect planning/review with Implementer coding through Git, .agent-team state, and Buzz handoff markers."
---

# Cooperative Development

Use this skill whenever this team works on a repository task.

## Start

1. Read `AGENTS.md` if present.
2. Read `.agent-team/PLAN.md`, `STATE.json`, `DECISIONS.md`, and `VERIFICATION.md` if present.
3. Run or inspect `git status` before assuming ownership of work.
4. Read the relevant source before relying on a chat summary.

## Shared state

The repository's `.agent-team/` directory is the durable handoff state. Buzz room messages stay concise and point to it.

`STATE.json` should use this shape when practical:

```json
{
  "task": "short-task-id",
  "phase": "planning|implementation|review|verification|complete|blocked",
  "owner": "architect|implementer|none",
  "baseCommit": "git-sha-or-null",
  "lastReviewedCommit": "git-sha-or-null",
  "completed": [],
  "remaining": [],
  "blockers": []
}
```

Do not fabricate a commit SHA. Use `null` when the repository state does not provide one.

## Architect protocol

- Find root cause and define invariants before implementation.
- Put the approved plan in `PLAN.md`.
- Handoff with `@Implementer [PLAN_READY]`.
- On return, compare the actual Git diff and fresh verification against the plan.
- Reply with `[REVIEW_PASS]` or `[REVIEW_FAIL]` and concrete evidence.

## Implementer protocol

- Implement the current plan rather than inventing a replacement architecture.
- Prefer a failing test before behavior-changing production code.
- Record exact verification commands/results in `VERIFICATION.md`.
- If repository evidence contradicts the plan, use `@Architect [PLAN_BLOCKED]`.
- When ready, use `@Architect [IMPLEMENTATION_READY]`.

## Evidence rule

Agent prose is not proof. Git diff and fresh verification output are proof. If they conflict, trust the repository evidence and investigate the discrepancy.
