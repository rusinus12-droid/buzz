# Solo Dev — Codex + Hermes

This fork ships a two-agent development team that keeps planning/review and implementation on separate runtimes while using one Buzz room as the collaboration surface.

## Roles

- **Architect** — runtime: `codex`
- **Implementer** — runtime: `hermes`

The pack deliberately does **not** hard-code model IDs. Pick the current Codex model you want for Architect and an authenticated Hermes model (for example an Ollama Cloud model) for Implementer from Buzz's existing agent/model controls.

## One-time setup in Buzz

1. In **Settings → Runtimes**, make sure Codex and Hermes are available.
   - Codex uses the normal Codex CLI login/ChatGPT subscription credential store.
   - Hermes uses the normal Hermes installation and ACP launcher (`hermes-acp`).
2. Configure the active Community's **repos directory** to the parent folder that already contains your local source checkouts (for example `/Users/you/Projects`). Buzz exposes it to managed agents as `REPOS/` without copying the repositories.
3. In **Agents**, edit the seeded **Architect** and **Implementer** definitions if you want to pin models.
   - Architect keeps runtime `codex`.
   - Implementer keeps runtime `hermes`; Hermes ACP model discovery can surface configured Ollama Cloud models in the normal Buzz model picker.
4. Deploy **Solo Dev — Codex + Hermes** to the project/channel where you want the pair to work.

After this, day-to-day work stays in Buzz. You should not need to launch a separate Hermes terminal session merely to participate in the team.

## Normal flow

1. Mention **Architect** with the task.
2. Architect resolves the repository under `REPOS/`, analyzes it, writes `.agent-team/PLAN.md` and state, then posts `@Implementer [PLAN_READY]`.
3. Implementer verifies the same repository, implements/tests the plan, records evidence, then posts `@Architect [IMPLEMENTATION_READY]`.
4. Architect reviews the real Git diff and verification evidence.
5. A failed review returns `@Implementer [REVIEW_FAIL]`; a passing review ends with `[REVIEW_PASS]`.

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
