# Solo Dev — Codex + Hermes

This fork ships a two-agent development team that keeps planning/review and implementation on separate runtimes while using one Buzz room as the collaboration surface.

## Roles

- **Architect** — runtime: `codex`, initial Codex agent mode: `read-only` (Codex ACP's conservative **Ask for approval** preset)
- **Implementer** — runtime: `hermes`, default reasoning effort: `high`

The pack deliberately does **not** hard-code model IDs. Pick the current Codex model you want for Architect and an authenticated Hermes model (for example an Ollama Cloud model) for Implementer from Buzz's existing agent/model controls.

Buzz projects each role's selected model through `BUZZ_ACP_MODEL`. `buzz-acp` applies that model immediately after every new ACP session is created. This makes the Architect's selected Codex model and the Implementer's provider-qualified Hermes model (for example `ollama-cloud:<model>`) sticky per managed agent without changing either CLI's global default.

## One-time setup in Buzz

1. In **Settings → Runtimes**, make sure Codex and Hermes are available.
   - Codex uses the normal Codex CLI login/ChatGPT subscription credential store.
   - Hermes uses the normal Hermes installation and ACP launcher (`hermes-acp`).
   - Hermes ACP reuses `~/.hermes` credentials/config. Ollama Cloud therefore needs to be authenticated/configured in Hermes once; after that Buzz can discover its models over ACP.
2. Configure the active Community's **repos directory** to the parent folder that already contains your local source checkouts (for example `/Users/you/Projects`). Buzz exposes it to managed agents as `REPOS/` without copying the repositories.
3. In **Agents**, edit the seeded **Architect** and **Implementer** definitions to pin the models you want.
   - Architect keeps runtime `codex` and can be pinned to the desired Codex model (for this workflow, Astra). `INITIAL_AGENT_MODE=read-only` selects Codex ACP's conservative approval/network posture, but the sandbox is still workspace-write so the Architect can create `.agent-team` files. The team protocol—not filesystem immutability—keeps normal production-code edits assigned to Implementer.
   - Implementer keeps runtime `hermes`; Hermes ACP model discovery can surface configured Ollama Cloud models in the normal Buzz model picker.
   - Implementer starts at `high` reasoning effort. Raise it to `max` from the managed-agent Thinking effort control for unusually hard fixes; the effort is applied at ACP session creation.
4. Deploy **Solo Dev — Codex + Hermes** to the project/channel where you want the pair to work.
   - Solo Dev intentionally disables Buzz's normal missing-runtime fallback. If either role's configured runtime is unavailable (or unset), deployment is blocked instead of silently replacing that role with another runtime.
   - This protects the core contract: Architect stays Codex and Implementer stays Hermes unless you explicitly edit their definitions.

After this, day-to-day work stays in Buzz. You should not need to launch a separate Hermes terminal session merely to participate in the team.

## Normal flow

1. Mention **Architect** with the task.
2. Architect resolves the repository under `REPOS/`, analyzes it, writes `.agent-team/PLAN.md` and state, then publishes a real Buzz mention to Implementer with `[PLAN_READY]`.
3. Implementer verifies the same repository, implements/tests the plan, records evidence, then publishes a real Buzz mention to Architect with `[IMPLEMENTATION_READY]`.
4. Architect reviews the real Git diff and verification evidence.
5. A failed review publishes `[REVIEW_FAIL]` back to Implementer; a passing review ends with `[REVIEW_PASS]`.

Handoffs are sent through `buzz messages send` in the current channel. A plain response that only contains `@Architect` or `@Implementer` is not treated as proof that the teammate was notified.

Both managed agents stay **Owner only**. Buzz ACP authorizes NIP-OA sibling agents with the same owner, so the two teammates can hand work to each other without opening them to arbitrary channel authors.

## Repository state

The team stores curated cross-session state inside the target repository:

```text
.agent-team/
├── PLAN.md
├── STATE.json
├── DECISIONS.md
└── VERIFICATION.md
```

Git/working-tree state is authoritative. `.agent-team/` is the curated handoff state, and Buzz room history is the detailed conversation log.
