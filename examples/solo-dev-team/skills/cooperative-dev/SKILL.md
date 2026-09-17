---
name: "cooperative-dev"
description: "Coordinate Architect planning/review with Implementer coding through Git, .agent-team state, and notifying Buzz handoffs."
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
  "phase": "planning|implementation|review|complete|blocked",
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

Normal state transitions are:

```text
planning / architect
  -> implementation / implementer
  -> review / architect
  -> implementation / implementer   (REVIEW_FAIL only)
  -> review / architect
  -> complete / none                (REVIEW_PASS)
```

A plan contradiction may move the task to `blocked / architect`. Do not edit production source while state ownership points to the other role unless the handoff explicitly explains the transition.

## Handoff transport

Do not rely on a plain assistant response containing `@Name`. A role handoff must be a real Buzz message in the current channel so it creates the signed mention that wakes the teammate.

- Use `buzz messages send` with the channel UUID from the current `<context>`.
- Use the teammate's exact current Buzz display name in the `@mention` text.
- When the teammate pubkey is known, pass it with `--mention`; otherwise exact current-channel member-name resolution may be used.
- Check the successful command result: the intended teammate should appear in `mention_pubkeys`.
- Do not send acknowledgement-only callback mentions; send only a plan, blocker, implementation result, or failed-review correction that requires action.
- `[REVIEW_PASS]` is terminal: publish it without an agent mention so no completed task wakes another model turn.

## Architect protocol

- Find root cause and define invariants before implementation.
- Put the approved plan in `PLAN.md`.
- Record the selected repository in `STATE.json` with `planning / architect` while planning.
- Before handoff, change state to `implementation / implementer`, then publish `@Implementer [PLAN_READY]` through `buzz messages send` in the current channel.
- On return, set state to `review / architect` and compare the actual Git diff plus fresh verification against the plan.
- On failure, return state to `implementation / implementer` and publish `@Implementer [REVIEW_FAIL]` with concrete evidence.
- On success, set state to `complete / none` and publish a channel-visible `[REVIEW_PASS]` result without mentioning Implementer.

## Implementer protocol

- Confirm the handoff resolves to the repository recorded in `STATE.json` and ownership is `implementer`.
- Implement the current plan rather than inventing a replacement architecture.
- Prefer a failing test before behavior-changing production code.
- Record exact verification commands/results in `VERIFICATION.md`.
- If repository evidence contradicts the plan, set `blocked / architect` and publish `@Architect [PLAN_BLOCKED]` through `buzz messages send`.
- When ready, set `review / architect` and publish `@Architect [IMPLEMENTATION_READY]` through `buzz messages send`.

## Evidence rule

Agent prose is not proof. Git diff and fresh verification output are proof. If they conflict, trust the repository evidence and investigate the discrepancy.
