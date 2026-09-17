---
name: "cooperative-dev"
description: "Coordinate Architect planning/review with Implementer coding through Git, .agent-team state, and Buzz handoff markers."
---

# Cooperative Development

Use this skill whenever this team works on a repository task.

## Resolve the repository first

Buzz agents normally start in the Buzz workspace. Existing source checkouts are exposed through `REPOS/`, which may be a symlink to the active Community's configured `reposDir`.

Resolve exactly one repository before reading or writing task state:

1. Prefer the repository identified in current Buzz project/channel context.
2. Otherwise use an exact repository/path named by the user or handoff.
3. Otherwise inspect only immediate children of `REPOS/` for an unambiguous match.
4. Never recursively scan the user's home directory to guess a checkout.

Once resolved, use the repository root as `workdir` for all repository commands. All `.agent-team/` paths below are relative to that root.

## Start

1. Resolve the target repository.
2. Read the repository's `AGENTS.md` if present.
3. Read `.agent-team/PLAN.md`, `STATE.json`, `DECISIONS.md`, and `VERIFICATION.md` if present.
4. Run or inspect `git status` before assuming ownership of work.
5. Read the relevant source before relying on a chat summary.

## Shared state

The repository's `.agent-team/` directory is the durable handoff state. Buzz room messages stay concise and point to it.

`STATE.json` should use this shape when practical:

```json
{
  "task": "short-task-id",
  "phase": "planning|implementation|review|verification|complete|blocked",
  "owner": "architect|implementer|none",
  "repoRoot": "absolute-or-REPOS-relative-path",
  "baseCommit": "git-sha-or-null",
  "lastReviewedCommit": "git-sha-or-null",
  "completed": [],
  "remaining": [],
  "blockers": []
}
```

Do not fabricate a path or commit SHA. Use `null` when the repository state does not provide one. The Implementer must treat a `repoRoot` mismatch as a blocker rather than editing another checkout.

## Architect protocol

- Find root cause and define invariants before implementation.
- Put the approved plan in `PLAN.md`.
- Record the selected repository in `STATE.json`.
- Handoff with `@Implementer [PLAN_READY]`.
- On return, compare the actual Git diff and fresh verification against the plan.
- Reply with `[REVIEW_PASS]` or `[REVIEW_FAIL]` and concrete evidence.

## Implementer protocol

- Confirm the handoff resolves to the repository recorded in `STATE.json`.
- Implement the current plan rather than inventing a replacement architecture.
- Prefer a failing test before behavior-changing production code.
- Record exact verification commands/results in `VERIFICATION.md`.
- If repository evidence contradicts the plan, use `@Architect [PLAN_BLOCKED]`.
- When ready, use `@Architect [IMPLEMENTATION_READY]`.

## Evidence rule

Agent prose is not proof. Git diff and fresh verification output are proof. If they conflict, trust the repository evidence and investigate the discrepancy.
