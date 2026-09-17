use std::collections::BTreeMap;

use tauri::AppHandle;

use super::{AcpSessionPolicy, AgentDefinition};

pub(crate) const SOLO_DEV_TEAM_ID: &str = "builtin-team:solo-dev";
pub(crate) const ARCHITECT_PERSONA_ID: &str = "solo-dev:architect";
pub(crate) const IMPLEMENTER_PERSONA_ID: &str = "solo-dev:implementer";

const ARCHITECT_PERSONA_MD: &str = include_str!(
    "../../../../examples/solo-dev-team/agents/architect.persona.md"
);
const IMPLEMENTER_PERSONA_MD: &str = include_str!(
    "../../../../examples/solo-dev-team/agents/implementer.persona.md"
);
const TEAM_INSTRUCTIONS: &str = include_str!("../../../../examples/solo-dev-team/instructions.md");

fn definition_from_persona_md(
    id: &str,
    content: &str,
    now: &str,
) -> Result<AgentDefinition, String> {
    let persona = buzz_persona_pkg::persona::parse_persona_md(content)
        .map_err(|error| format!("failed to parse Solo Dev persona {id}: {error}"))?;

    let system_prompt = format!(
        "{}\n\n---\n# Team Instructions\n\n{}",
        persona.prompt.trim(),
        TEAM_INSTRUCTIONS.trim()
    );

    Ok(AgentDefinition {
        id: id.to_string(),
        display_name: persona.display_name,
        avatar_url: None,
        description: Some(persona.description),
        system_prompt,
        runtime: persona.runtime,
        // Deliberately leave model/provider unset. Codex uses the user's
        // ChatGPT/Codex subscription selection; Hermes exposes its own ACP
        // model picker (including authenticated Ollama Cloud models).
        model: None,
        provider: None,
        name_pool: vec![],
        // These are fork-provided team personas, not upstream built-ins. Keeping
        // them non-builtin prevents upstream `merge_personas` from demoting them
        // because it only knows the upstream built-in ID table.
        is_builtin: false,
        is_active: true,
        shared: false,
        source_team: Some(SOLO_DEV_TEAM_ID.to_string()),
        source_team_persona_slug: Some(persona.name),
        catalog_source: None,
        team_catalog_source: None,
        env_vars: BTreeMap::new(),
        // Hermes ACP can execute terminal commands and Buzz may answer ACP
        // approval requests automatically. Solo Dev agents therefore default
        // to owner-only access even if a future build relaxes the global clamp.
        respond_to: Some("owner-only".to_string()),
        respond_to_allowlist: vec![],
        parallelism: Some(1),
        session_policy: AcpSessionPolicy::default(),
        created_at: now.to_string(),
        updated_at: now.to_string(),
    })
}

pub(crate) fn solo_dev_persona_records(now: &str) -> Result<Vec<AgentDefinition>, String> {
    Ok(vec![
        definition_from_persona_md(ARCHITECT_PERSONA_ID, ARCHITECT_PERSONA_MD, now)?,
        definition_from_persona_md(IMPLEMENTER_PERSONA_ID, IMPLEMENTER_PERSONA_MD, now)?,
    ])
}

/// Ensure the fork's two cooperative personas exist without replacing an
/// existing record. This is intentionally idempotent: runtime/model choices or
/// future local edits stored on an existing definition remain untouched.
pub(crate) fn ensure_solo_dev_personas<R: tauri::Runtime>(
    app: &AppHandle<R>,
) -> Result<bool, String> {
    let now = crate::util::now_iso();
    let mut personas = super::load_personas(app)?;
    let mut changed = false;

    for seed in solo_dev_persona_records(&now)? {
        if personas.iter().any(|record| record.id == seed.id) {
            continue;
        }
        personas.push(seed);
        changed = true;
    }

    if changed {
        super::save_personas(app, &personas)?;
    }

    Ok(changed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn definitions_route_architecture_to_codex_and_implementation_to_hermes() {
        let records = solo_dev_persona_records("2026-09-17T00:00:00Z").unwrap();
        assert_eq!(records.len(), 2);

        let architect = records
            .iter()
            .find(|record| record.id == ARCHITECT_PERSONA_ID)
            .unwrap();
        assert_eq!(architect.runtime.as_deref(), Some("codex"));
        assert!(architect.model.is_none());
        assert!(architect.provider.is_none());
        assert_eq!(architect.respond_to.as_deref(), Some("owner-only"));
        assert!(architect.system_prompt.contains("[PLAN_READY]"));

        let implementer = records
            .iter()
            .find(|record| record.id == IMPLEMENTER_PERSONA_ID)
            .unwrap();
        assert_eq!(implementer.runtime.as_deref(), Some("hermes"));
        assert!(implementer.model.is_none());
        assert!(implementer.provider.is_none());
        assert_eq!(implementer.respond_to.as_deref(), Some("owner-only"));
        assert!(implementer.system_prompt.contains("[IMPLEMENTATION_READY]"));
    }

    #[test]
    fn both_personas_share_the_same_durable_handoff_protocol() {
        let records = solo_dev_persona_records("2026-09-17T00:00:00Z").unwrap();
        for record in records {
            assert!(record.system_prompt.contains(".agent-team/"));
            assert!(record.system_prompt.contains("[PLAN_BLOCKED]"));
            assert!(record.system_prompt.contains("[REVIEW_FAIL]"));
            assert!(record.system_prompt.contains("[REVIEW_PASS]"));
        }
    }
}
