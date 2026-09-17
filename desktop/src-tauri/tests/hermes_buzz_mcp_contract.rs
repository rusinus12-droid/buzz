//! Regression contracts for the Buzz ↔ Hermes managed-runtime bridge.
//!
//! Hermes must be represented in the first-class runtime catalog with
//! `buzz-dev-mcp`. The managed-agent spawn path derives MCP injection from
//! `known_acp_runtime()`, so leaving Hermes only in the Tier-2 preset table
//! causes ACP sessions to start with `mcpServers: []` and prevents Hermes from
//! publishing its reply back into a Buzz room.
//!
//! The persisted model must also project to `BUZZ_ACP_MODEL`. buzz-acp reads
//! that value and applies it to every new ACP session, so a provider-qualified
//! Hermes model such as `ollama-cloud:<model>` remains sticky without requiring
//! the user to launch Hermes separately in an Ollama mode.
//!
//! The legacy Tier-2 Hermes preset may remain for compatibility: discovery
//! seeds built-in IDs first and skips a preset whose ID is already present.

const CATALOG_SOURCE: &str = include_str!("../src/managed_agents/discovery/catalog.rs");

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
    assert!(
        hermes.contains("supports_acp_model_switching: true"),
        "Hermes model switching should remain owned by the ACP session"
    );
    assert!(
        hermes.contains("model_env_var: Some(\"BUZZ_ACP_MODEL\")"),
        "the persona's provider-qualified Hermes model must be applied to every new ACP session"
    );
    assert!(
        hermes.contains("HERMES_ACP_SKIP_CONFIGURED_MCP"),
        "Buzz should own the session MCP list without starting unrelated global Hermes MCP servers"
    );
}
