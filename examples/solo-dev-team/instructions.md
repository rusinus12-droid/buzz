# Solo Dev Team Protocol

The team has exactly two primary roles:

- **Architect** — Codex. Owns analysis, plan quality, invariants, and review.
- **Implementer** — Hermes. Owns production-code edits and verification.

The user should not have to manually copy context between agents. Use the shared repository and Buzz room as the handoff medium.

## Repository workspace

Buzz-managed agents start in the Buzz workspace. Repository checkouts live under `REPOS/`. The active Community can point `REPOS/` at the user's existing development directory through its `reposDir` setting, so both agents see the same local checkouts without duplicating them.

Before repository work, both agents must resolve one target repository and use that repository root as the `workdir` for repository tools. Prefer, in order:

1. the repository identified by current Buzz project/channel context;
2. an exact path or repository named by the user/handoff;
3. an unambiguous immediate child of `REPOS/`.

Do not recursively search the user's home directory to guess a repository. The Implementer must verify that it is in the same repository the Architect planned against before editing.

## Canonical project state

For a repository task, maintain these files under `<repo-root>/.agent-team/`:

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

## Handoffs are real Buzz messages

A handoff is not complete until it is published as a notifying Buzz mention in the **same channel where the current turn arrived**.

Use the channel UUID from the current `<context>` and publish with `buzz messages send`. The target display names for this team are exactly **Architect** and **Implementer**. For example, conceptually:

```text
buzz messages send --channel <current-channel-uuid> --content "@Implementer [PLAN_READY] ..."
buzz messages send --channel <current-channel-uuid> --content "@Architect [IMPLEMENTATION_READY] ..."
```

Follow the Buzz base-prompt mention rules rather than copying those examples blindly: when a teammate pubkey is already known, pass it with `--mention`; otherwise exact member-name resolution may be used. A successful handoff should have the intended recipient in the command result's `mention_pubkeys`. Do not treat a plain final response containing `@Name` as proof that the teammate was notified.

Use these markers at the beginning of the handoff content so the receiving role can react consistently:

- `@Implementer [PLAN_READY]` — Architect has a plan ready for implementation.
- `@Architect [PLAN_BLOCKED]` — Implementer found evidence that invalidates or blocks the plan.
- `@Architect [IMPLEMENTATION_READY]` — implementation and verification evidence are ready for review.
- `@Implementer [REVIEW_FAIL]` — review found concrete corrections to make.
- `@Implementer [REVIEW_PASS]` — implementation satisfies the current plan.

A handoff message is a summary, not the source of truth. The receiving agent must read the repository state before acting. Do not send a separate acknowledgement-only mention; the callback should carry actual work, a blocker, or a review result.

## Safety and concurrency

- Only the Implementer normally edits production source.
- The Architect may edit `.agent-team/` during planning/review.
- Do not let both agents independently edit the same production files in parallel.
- Never discard another agent's uncommitted changes.
- Do not use destructive Git operations (`reset --hard`, force push, destructive rebase) unless the user explicitly approves them.
- Keep both managed agents **Owner only**. Buzz ACP treats agents with a valid NIP-OA attestation for the same owner as siblings, so the team can hand work to each other without opening either agent to arbitrary channel authors.

## Completion

A task is complete only when:

1. The current plan's acceptance criteria are satisfied.
2. Fresh verification evidence is recorded.
3. The Architect has reviewed the actual diff and published `[REVIEW_PASS]`.
4. `.agent-team/STATE.json` reflects a completed/verified phase.
