#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

printf '\n==> Solo Dev persona pack\n'
cargo test -p buzz-persona --test solo_dev_pack

printf '\n==> Solo Dev desktop runtime contracts\n'
cargo test \
  --manifest-path desktop/src-tauri/Cargo.toml \
  --test hermes_buzz_mcp_contract \
  --test solo_dev_team_contract

printf '\n==> Solo Dev team runtime guard\n'
node \
  --import ./desktop/test-loader.mjs \
  --experimental-strip-types \
  --test desktop/src/features/agents/lib/soloDevRuntimeGuard.test.mjs

printf '\nSolo Dev harness contract checks passed.\n'
