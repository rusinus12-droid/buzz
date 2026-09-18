#!/usr/bin/env bash
set -euo pipefail

if [[ "$(uname -s)" != "Darwin" ]]; then
  echo "This smoke check is intended for macOS." >&2
  exit 2
fi

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

# Buzz pins its development toolchain through Hermit. Keep this script on the
# same PATH/tool versions as the repository's normal contributor commands.
if [[ -f "$ROOT/bin/activate-hermit" ]]; then
  # shellcheck disable=SC1091
  . "$ROOT/bin/activate-hermit"
fi

require_cmd() {
  local name="$1"
  if ! command -v "$name" >/dev/null 2>&1; then
    echo "Missing required command: $name" >&2
    return 1
  fi
  printf '%-12s %s\n' "$name" "$(command -v "$name")"
}

printf '\n==> Runtime executables\n'
require_cmd codex
require_cmd codex-acp
require_cmd hermes
require_cmd hermes-acp

printf '\n==> Codex subscription/login status\n'
codex login status

printf '\n==> Hermes ACP installation\n'
hermes acp --check

printf '\n==> Hermes reasoning baseline\n'
HERMES_REASONING="$(hermes config get agent.reasoning_effort 2>/dev/null || true)"
if ! printf '%s\n' "$HERMES_REASONING" | grep -Eqi '(^|[^a-z])high([^a-z]|$)'; then
  echo "Hermes agent.reasoning_effort is not high." >&2
  echo "Current value: ${HERMES_REASONING:-<unset>}" >&2
  echo "Set the Solo Dev baseline once with:" >&2
  echo "  hermes config set agent.reasoning_effort high" >&2
  exit 1
fi
echo "Hermes reasoning effort: high"

printf '\n==> Build local buzz-acp probe\n'
cargo build -p buzz-acp
BUZZ_ACP_BIN="$ROOT/target/debug/buzz-acp"
if [[ ! -x "$BUZZ_ACP_BIN" ]]; then
  echo "buzz-acp build completed but binary was not found at $BUZZ_ACP_BIN" >&2
  exit 1
fi

TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT

printf '\n==> Codex ACP model discovery\n'
BUZZ_ACP_AGENT_COMMAND="$(command -v codex-acp)" \
  "$BUZZ_ACP_BIN" models --json >"$TMP_DIR/codex-models.json"
if ! grep -q '"models"' "$TMP_DIR/codex-models.json"; then
  echo "Codex ACP returned no model catalog." >&2
  exit 1
fi

echo "Codex ACP model catalog: OK"

printf '\n==> Hermes ACP model discovery\n'
HERMES_ACP_SKIP_CONFIGURED_MCP=1 \
BUZZ_ACP_AGENT_COMMAND="$(command -v hermes-acp)" \
  "$BUZZ_ACP_BIN" models --json >"$TMP_DIR/hermes-models.json"
if ! grep -q '"models"' "$TMP_DIR/hermes-models.json"; then
  echo "Hermes ACP returned no model catalog." >&2
  exit 1
fi
if ! grep -qi 'ollama-cloud' "$TMP_DIR/hermes-models.json"; then
  echo "Hermes ACP is working, but no Ollama Cloud model is visible." >&2
  echo "Configure/authenticate Ollama Cloud in Hermes once, then rerun this check." >&2
  exit 1
fi

echo "Hermes ACP Ollama Cloud catalog: OK"

printf '\nSolo Dev macOS runtime smoke checks passed.\n'
printf 'Next: run scripts/verify-solo-dev-harness.sh, then launch the Buzz Desktop fork for the live handoff test.\n'
