# Solo Dev Team Protocol

The team has exactly two primary roles:

- **Architect** — Codex. Owns analysis, plan quality, invariants, and review.
- **Implementer** — Hermes. Owns production-code edits and verification.

The user should not have to manually copy context between agents. Use the shared repository and Buzz room as the handoff medium.

## Canonical project state

For a repository task, maintain these files under `.agent-team/`:

- `PLAN.md` — current approved design and acceptance criteria.
- `STATE.json` — machine-readable current phase, owner, Git baseline, completed work, and remaining work.
- `DECISIONS.md` — durable architecture decisions and reasons.
- `VERIFICATION.md` — exact test/lint/build commands and their latest results.

Create the directory/files when they are needed and absent. Keep them concise; Buzz room history is the detailed conversation log.

Authority order:

1. **Git / working tree** — what actually changed.
2. **`.agent-team/`** — curated current intent and state.
3. **Buzz room history** — discussion and progress narrative.

If they disagree, investigate and reconcile them. Never overwrite reality with a stale status document.

## Handoff markers

Use these markers at the beginning of a handoff message so the next role can react consistently:

- `@Implementer [PLAN_READY]` — Architect has a plan ready for implementation.
- `@Architect [PLAN_BLOCKED]` — Implementer found evidence that invalidates or blocks the plan.
- `@Architect [IMPLEMENTATION_READY]` — implementation and verification evidence are ready for review.
- `@Implementer [REVIEW_FAIL]` — review found concrete corrections to make.
- `@Implementer [REVIEW_PASS]` — implementation satisfies the current plan.

A handoff message is a summary, not the source of truth. The receiving agent must read the repository state before acting.

## Safety and concurrency

- Only the Implementer normally edits production source.
- The Architect may edit `.agent-team/` during planning/review.
- Do not let both agents independently edit the same production files in parallel.
- Never discard another agent's uncommitted changes.
- Do not use destructive Git operations (`reset --hard`, force push, destructive rebase) unless the user explicitly approves them.
- Keep Buzz-managed Hermes agents **Owner only** because the ACP host can auto-approve shell permission requests.

## Completion

A task is complete only when:

1. The current plan's acceptance criteria are satisfied.
2. Fresh verification evidence is recorded.
3. The Architect has reviewed the actual diff and posted `[REVIEW_PASS]`.
4. `.agent-team/STATE.json` reflects a completed/verified phase.
