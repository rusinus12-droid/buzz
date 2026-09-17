const TEAMS_SOURCE: &str = include_str!("../src/managed_agents/teams.rs");
const SOLO_DEV_SOURCE: &str = include_str!("../src/managed_agents/solo_dev.rs");

#[test]
fn built_in_team_catalog_exposes_solo_dev_pair() {
    assert!(TEAMS_SOURCE.contains("SOLO_DEV_TEAM_ID"));
    assert!(TEAMS_SOURCE.contains("ARCHITECT_PERSONA_ID"));
    assert!(TEAMS_SOURCE.contains("IMPLEMENTER_PERSONA_ID"));
}

#[test]
fn normal_team_load_bootstraps_solo_dev_personas() {
    assert!(
        TEAMS_SOURCE.contains("ensure_solo_dev_personas(app)?"),
        "loading teams should ensure the fork-provided Architect/Implementer definitions exist"
    );
}

#[test]
fn solo_dev_bootstrap_embeds_the_pack_as_its_single_prompt_source() {
    assert!(SOLO_DEV_SOURCE.contains("include_str!"));
    assert!(SOLO_DEV_SOURCE.contains("architect.persona.md"));
    assert!(SOLO_DEV_SOURCE.contains("implementer.persona.md"));
    assert!(SOLO_DEV_SOURCE.contains("instructions.md"));
}
