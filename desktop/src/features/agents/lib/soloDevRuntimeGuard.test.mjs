import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import test from "node:test";

import {
  getSoloDevRuntimeGuard,
  SOLO_DEV_TEAM_ID,
  UNCONFIGURED_RUNTIME_ID,
} from "./soloDevRuntimeGuard.ts";

test("ordinary teams keep generic runtime fallback behavior", () => {
  const guard = getSoloDevRuntimeGuard(
    "team:ordinary",
    [{ runtime: "codex" }, { runtime: "hermes" }],
    [{ id: "codex" }],
  );

  assert.deepEqual(guard, { strict: false, missingRuntimeIds: [] });
});

test("Solo Dev requires every persona runtime to be available", () => {
  const guard = getSoloDevRuntimeGuard(
    SOLO_DEV_TEAM_ID,
    [{ runtime: "codex" }, { runtime: "hermes" }],
    [{ id: "codex" }],
  );

  assert.deepEqual(guard, {
    strict: true,
    missingRuntimeIds: ["hermes"],
  });
});

test("Solo Dev is deployable when both requested runtimes are available", () => {
  const guard = getSoloDevRuntimeGuard(
    SOLO_DEV_TEAM_ID,
    [{ runtime: "codex" }, { runtime: "hermes" }],
    [{ id: "hermes" }, { id: "codex" }],
  );

  assert.deepEqual(guard, { strict: true, missingRuntimeIds: [] });
});

test("Solo Dev blocks blank runtime configuration and deduplicates ids", () => {
  const guard = getSoloDevRuntimeGuard(
    SOLO_DEV_TEAM_ID,
    [{ runtime: "hermes" }, { runtime: "hermes" }, { runtime: null }],
    [],
  );

  assert.deepEqual(guard.missingRuntimeIds, [
    "hermes",
    UNCONFIGURED_RUNTIME_ID,
  ]);
});

test("team deployment UI consumes the strict Solo Dev guard", () => {
  const source = readFileSync(
    new URL("../ui/AddTeamToChannelDialog.tsx", import.meta.url),
    "utf8",
  );

  assert.match(source, /getSoloDevRuntimeGuard\(team\?\.id, resolved, runtimes\)/);
  assert.match(source, /soloDevRuntimeBlocked/);
  assert.match(source, /Runtime fallback is disabled for this team/);
  assert.match(
    source,
    /runtime\.id === \(persona\.runtime\?\.trim\(\) \?\? ""\)/,
    "Solo Dev deploy must use each persona's exact configured runtime rather than default fallback",
  );
  assert.match(
    source,
    /if \(!runtimeToUse\) \{\s*return;/,
    "deployment should fail closed if runtime availability changes between render and click",
  );
  assert.doesNotMatch(
    source,
    /throw new Error\(\s*`Solo Dev runtime/,
    "local strict-runtime validation should not be swallowed by the mutation error catch",
  );
});
