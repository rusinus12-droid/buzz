import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import test from "node:test";

import {
  getSoloDevRuntimeGuard,
  SOLO_DEV_ARCHITECT_PERSONA_ID,
  SOLO_DEV_IMPLEMENTER_PERSONA_ID,
  SOLO_DEV_TEAM_ID,
} from "./soloDevRuntimeGuard.ts";

const architect = (runtime = "codex") => ({
  id: SOLO_DEV_ARCHITECT_PERSONA_ID,
  runtime,
});
const implementer = (runtime = "hermes") => ({
  id: SOLO_DEV_IMPLEMENTER_PERSONA_ID,
  runtime,
});

test("ordinary teams keep generic runtime fallback behavior", () => {
  const guard = getSoloDevRuntimeGuard(
    "team:ordinary",
    [architect(), implementer()],
    [{ id: "codex" }],
  );

  assert.deepEqual(guard, {
    strict: false,
    missingRuntimeIds: [],
    roleContractErrors: [],
  });
});

test("Solo Dev requires both Codex and Hermes to be available", () => {
  const guard = getSoloDevRuntimeGuard(
    SOLO_DEV_TEAM_ID,
    [architect(), implementer()],
    [{ id: "codex" }],
  );

  assert.deepEqual(guard, {
    strict: true,
    missingRuntimeIds: ["hermes"],
    roleContractErrors: [],
  });
});

test("Solo Dev is deployable only with the exact two-role binding", () => {
  const guard = getSoloDevRuntimeGuard(
    SOLO_DEV_TEAM_ID,
    [architect(), implementer()],
    [{ id: "hermes" }, { id: "codex" }],
  );

  assert.deepEqual(guard, {
    strict: true,
    missingRuntimeIds: [],
    roleContractErrors: [],
  });
});

test("Solo Dev rejects collapsing Architect onto Hermes even when Hermes is available", () => {
  const guard = getSoloDevRuntimeGuard(
    SOLO_DEV_TEAM_ID,
    [architect("hermes"), implementer("hermes")],
    [{ id: "hermes" }, { id: "codex" }],
  );

  assert.deepEqual(guard.missingRuntimeIds, []);
  assert.deepEqual(guard.roleContractErrors, [
    "Architect must use codex; configured runtime is hermes.",
  ]);
});

test("Solo Dev rejects a team with the Implementer missing", () => {
  const guard = getSoloDevRuntimeGuard(
    SOLO_DEV_TEAM_ID,
    [architect()],
    [{ id: "codex" }, { id: "hermes" }],
  );

  assert.deepEqual(guard.missingRuntimeIds, []);
  assert.equal(guard.roleContractErrors.length, 2);
  assert.match(guard.roleContractErrors[0], /requires exactly 2 roles/);
  assert.equal(
    guard.roleContractErrors[1],
    "Implementer role is missing from the team.",
  );
});

test("Solo Dev rejects an unset required role runtime", () => {
  const guard = getSoloDevRuntimeGuard(
    SOLO_DEV_TEAM_ID,
    [architect(""), implementer()],
    [{ id: "codex" }, { id: "hermes" }],
  );

  assert.deepEqual(guard.roleContractErrors, [
    "Architect must use codex; configured runtime is (unset).",
  ]);
});

test("team deployment UI consumes the strict Solo Dev guard", () => {
  const source = readFileSync(
    new URL("../ui/AddTeamToChannelDialog.tsx", import.meta.url),
    "utf8",
  );

  assert.match(source, /getSoloDevRuntimeGuard\(team\?\.id, resolved, runtimes\)/);
  assert.match(source, /soloDevRuntimeGuard\.roleContractErrors\.length > 0/);
  assert.match(source, /Runtime fallback is disabled for this team/);
  assert.match(
    source,
    /runtime\.id === \(persona\.runtime\?\.trim\(\) \?\? ""\)/,
    "Solo Dev deploy must use each persona's exact configured runtime rather than default fallback",
  );
  assert.match(
    source,
    /const inputs: CreateChannelManagedAgentInput\[\] = \[\];/,
    "deployment inputs should retain the channel-agent input contract under strict TypeScript",
  );
  assert.match(
    source,
    /if \(!runtimeToUse\) \{\s*return;/,
    "deployment should fail closed if runtime availability changes between render and click",
  );
  assert.match(
    source,
    /forceNewInstance: !soloDevRuntimeGuard\.strict/,
    "Solo Dev redeploy should reuse team-bound role instances instead of creating duplicate mention names",
  );
});
