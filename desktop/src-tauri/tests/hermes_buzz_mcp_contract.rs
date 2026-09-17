//! Regression contract for Buzz issue #7023-style failures.
//!
//! Hermes must be represented in the first-class runtime catalog with
//! `buzz-dev-mcp`. The managed-agent spawn path derives MCP injection from
//! `known_acp_runtime()`, so leaving Hermes only in the Tier-2 preset table
//! causes ACP sessions to start with `mcpServers: []` and prevents Hermes from
//! publishing its reply back into a Buzz room.

const CATALOG_SOURCE: &str = include_str!("../src/managed_agents/discovery/catalog.rs");
const PRESETS_SOURCE: &str = include_str!("../src/managed_agents/discovery/presets.rs");

fn runtime_block<'a>(source: &'a str, id: &str) -> Option<&'a str> {
    let marker = format!("id: \"{id}\"");
    let after = source.split_once(&marker)?.1;
    Some(
        after
            .split("KnownAcpRuntime {")
            .next()
            .unwrap_or(after),
    )
}

#[test]
fn hermes_is_first_class_runtime_with_buzz_dev_mcp() {
    let hermes = runtime_block(CATALOG_SOURCE, "hermes")
        .expect("Hermes must be present in KNOWN_ACP_RUNTIMES");

    assert!(
        hermes.contains("mcp_command: Some(\"buzz-dev-mcp\")"),
        "Hermes must receive buzz-dev-mcp so it can publish replies to Buzz rooms"
    );
}

#[test]
fn hermes_is_not_duplicated_in_tier2_presets() {
    assert!(
        !PRESETS_SOURCE.contains("id: \"hermes\""),
        "Hermes must have one authoritative runtime definition; keep it out of PRESET_HARNESSES once promoted"
    );
}
