import assert from "node:assert/strict";
import test from "node:test";

import {
  HERMES_SETUP_AUTH_METHOD_ID,
  shouldDiscoverRuntimeAuthActions,
  visibleRuntimeAuthActions,
} from "./runtimeAuthActions.ts";

const runtime = (id, status = "not_applicable", availability = "available") => ({
  id,
  authStatus: { status },
  availability,
});

const methods = [
  { id: "ollama-cloud", name: "Ollama Cloud runtime credentials" },
  { id: HERMES_SETUP_AUTH_METHOD_ID, name: "Configure Hermes provider", type: "terminal" },
];

test("Hermes setup actions are discoverable even when generic auth is not applicable", () => {
  assert.equal(shouldDiscoverRuntimeAuthActions(runtime("hermes")), true);
});

test("unavailable Hermes does not probe auth actions", () => {
  assert.equal(
    shouldDiscoverRuntimeAuthActions(runtime("hermes", "not_applicable", "not_installed")),
    false,
  );
});

test("generic runtimes retain logged-out-only auth discovery", () => {
  assert.equal(shouldDiscoverRuntimeAuthActions(runtime("codex", "logged_out")), true);
  assert.equal(shouldDiscoverRuntimeAuthActions(runtime("codex", "logged_in")), false);
});

test("Hermes settings show only the terminal provider setup action", () => {
  assert.deepEqual(
    visibleRuntimeAuthActions("hermes", methods).map((method) => method.id),
    [HERMES_SETUP_AUTH_METHOD_ID],
  );
});

test("other runtimes keep their full advertised auth method list", () => {
  assert.deepEqual(
    visibleRuntimeAuthActions("codex", methods).map((method) => method.id),
    methods.map((method) => method.id),
  );
});
