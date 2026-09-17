use std::path::PathBuf;

use buzz_persona::{pack, validate};

fn solo_dev_pack_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/solo-dev-team")
}

#[test]
fn solo_dev_pack_is_valid_and_routes_roles_to_expected_runtimes() {
    let root = solo_dev_pack_root();

    let report = validate::validate_pack(&root);
    assert!(
        !report.has_errors(),
        "Solo Dev pack should validate cleanly, got: {report}"
    );

    let loaded = pack::load_pack(&root).expect("Solo Dev pack should load");
    assert_eq!(loaded.manifest.id, "dev.rusinus12.buzz-solo-dev");
    assert_eq!(loaded.personas.len(), 2);

    let architect = loaded
        .personas
        .iter()
        .find(|persona| persona.name == "architect")
        .expect("architect persona should exist");
    assert_eq!(architect.runtime.as_deref(), Some("codex"));
    assert_eq!(
        architect.model, None,
        "the pack must not hard-code a Codex model; the user's subscription/runtime selection owns it"
    );
    assert!(architect.prompt.contains("[PLAN_READY]"));
    assert!(architect.prompt.contains(".agent-team/PLAN.md"));
    assert!(architect.prompt.contains("REPOS/"));

    let implementer = loaded
        .personas
        .iter()
        .find(|persona| persona.name == "implementer")
        .expect("implementer persona should exist");
    assert_eq!(implementer.runtime.as_deref(), Some("hermes"));
    assert_eq!(
        implementer.model, None,
        "the pack must not hard-code a Hermes model; Buzz/Hermes model selection owns it"
    );
    assert!(implementer.prompt.contains("[IMPLEMENTATION_READY]"));
    assert!(implementer.prompt.contains(".agent-team/VERIFICATION.md"));
    assert!(implementer.prompt.contains("REPOS/"));

    let instructions = loaded
        .pack_instructions
        .as_deref()
        .expect("Solo Dev pack should carry shared instructions");
    assert!(instructions.contains("reposDir"));
    assert!(instructions.contains("<repo-root>/.agent-team/"));
    assert!(instructions.contains("buzz messages send"));
    assert!(instructions.contains("mention_pubkeys"));
    for marker in [
        "[PLAN_READY]",
        "[PLAN_BLOCKED]",
        "[IMPLEMENTATION_READY]",
        "[REVIEW_FAIL]",
        "[REVIEW_PASS]",
    ] {
        assert!(
            instructions.contains(marker),
            "shared instructions should define handoff marker {marker}"
        );
    }
}
