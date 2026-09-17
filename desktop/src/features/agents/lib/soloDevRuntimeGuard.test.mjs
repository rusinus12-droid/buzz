import assert from "node:assert/strict";
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
