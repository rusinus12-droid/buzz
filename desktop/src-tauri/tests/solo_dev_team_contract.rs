const TEAMS_SOURCE: &str = include_str!("../src/managed_agents/teams.rs");
const SOLO_DEV_SOURCE: &str = include_str!("../src/managed_agents/solo_dev.rs");
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
fn normal_team_load_bootstraps_solo_dev_extension() {
    assert!(
        TEAMS_SOURCE.contains("ensure_solo_dev_personas(app)?"),
        "loading teams should ensure the fork-provided Architect/Implementer definitions exist"
    );
    assert!(
        TEAMS_SOURCE.contains("ensure_solo_dev_team_record(&mut records, &now)"),
        "loading teams should seed the fork-specific Solo Dev team without changing upstream's built-in table"
    );
    assert!(
        TEAMS_SOURCE.contains("record.id != super::solo_dev::SOLO_DEV_TEAM_ID"),
        "upstream retirement logic must preserve the fork-specific built-in marker"
    );
}

#[test]
fn solo_dev_extension_owns_the_role_pair() {
    assert!(SOLO_DEV_SOURCE.contains("ARCHITECT_PERSONA_ID"));
    assert!(SOLO_DEV_SOURCE.contains("IMPLEMENTER_PERSONA_ID"));
    assert!(SOLO_DEV_SOURCE.contains("ensure_solo_dev_team_record"));
    assert!(SOLO_DEV_SOURCE.contains("INITIAL_AGENT_MODE"));
    assert!(!SOLO_DEV_SOURCE.contains("BUZZ_ACP_EFFORT_LEVEL"));
    assert!(
        SOLO_DEV_SOURCE.contains("\"read-only\""),
        "Architect should start in the conservative Codex ACP mode"
    );
    assert!(
        CATALOG_SOURCE.contains("thinking_env_var: None"),
        "Hermes reasoning should remain owned by Hermes config until ACP exposes a real effort control"
    );
}

#[test]
fn codex_architect_model_is_projected_to_acp_session_startup() {
    let codex = runtime_block(CATALOG_SOURCE, "codex")
        .expect("Codex must be present in KNOWN_ACP_RUNTIMES");
    assert!(
        codex.contains("supports_acp_model_switching: true"),
        "current codex-acp exposes stable model configOptions"
    );
    assert!(
        codex.contains("model_env_var: Some(\"BUZZ_ACP_MODEL\")"),
        "the Architect definition's selected Codex model must be applied after every session/new"
    );
}

#[test]
fn solo_dev_bootstrap_embeds_the_pack_as_its_single_prompt_source() {
    assert!(SOLO_DEV_SOURCE.contains("include_str!"));
    assert!(SOLO_DEV_SOURCE.contains("architect.persona.md"));
    assert!(SOLO_DEV_SOURCE.contains("implementer.persona.md"));
    assert!(SOLO_DEV_SOURCE.contains("instructions.md"));
}
