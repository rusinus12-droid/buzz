use std::collections::BTreeMap;

use tauri::AppHandle;

use super::{AcpSessionPolicy, AgentDefinition, TeamRecord};

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

    let mut env_vars = BTreeMap::new();
    if id == ARCHITECT_PERSONA_ID {
        // codex-acp officially supports INITIAL_AGENT_MODE. Keep the planning
        // role in read-only mode at the runtime boundary, not merely by prompt,
        // so production source writes remain the Implementer's responsibility.
        env_vars.insert("INITIAL_AGENT_MODE".to_string(), "read-only".to_string());
    }

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
        // Fork-provided definitions stay user-editable so the user can select
        // the exact Codex/Hermes model in Buzz. The built-in team references
        // these ids, which protects them from deletion while the team exists;
        // do not mark them as directory-backed team personas (`source_team`),
        // because Buzz intentionally locks model/system-prompt editing for
        // that imported-team shape.
        is_builtin: false,
        is_active: true,
        shared: false,
        source_team: None,
        source_team_persona_slug: None,
        catalog_source: None,
        team_catalog_source: None,
        env_vars,
        // Hermes ACP can execute terminal commands and Buzz may answer ACP
        // approval requests automatically. Solo Dev agents therefore default
        // to owner-only access even if a future build relaxes the global clamp.
        // Buzz ACP's owner gate also accepts NIP-OA siblings owned by the same
        // user, so Architect ↔ Implementer handoffs remain possible.
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

/// Add the fork-specific Solo Dev team to the normal mutable team load without
/// changing upstream's `BUILT_IN_TEAMS` table. Keeping the extension at this
/// seam avoids rewriting upstream tests and makes future rebases smaller.
/// Existing user customizations are preserved; only a stale `is_builtin`
/// marker is restored so the team cannot be deleted accidentally.
pub(crate) fn ensure_solo_dev_team_record(records: &mut Vec<TeamRecord>, now: &str) -> bool {
    if let Some(existing) = records.iter_mut().find(|team| team.id == SOLO_DEV_TEAM_ID) {
        if existing.is_builtin {
            return false;
        }
        existing.is_builtin = true;
        existing.updated_at = now.to_string();
        return true;
    }

    records.push(TeamRecord {
        id: SOLO_DEV_TEAM_ID.to_string(),
        name: "Solo Dev — Codex + Hermes".to_string(),
        description: Some(
            "Codex plans and reviews while Hermes implements and verifies in the same Buzz room."
                .to_string(),
        ),
        // Shared instructions already live in both persona prompts so they also
        // survive standalone persona use; do not inject the same text twice.
        instructions: None,
        persona_ids: vec![
            ARCHITECT_PERSONA_ID.to_string(),
            IMPLEMENTER_PERSONA_ID.to_string(),
        ],
        is_builtin: true,
        shared: false,
        catalog_source: None,
        source_dir: None,
        is_symlink: false,
        symlink_target: None,
        version: Some("0.1.0".to_string()),
        created_at: now.to_string(),
        updated_at: now.to_string(),
    });
    true
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
        assert_eq!(
            architect.env_vars.get("INITIAL_AGENT_MODE").map(String::as_str),
            Some("read-only")
        );
        assert!(architect.system_prompt.contains("[PLAN_READY]"));

        let implementer = records
            .iter()
            .find(|record| record.id == IMPLEMENTER_PERSONA_ID)
            .unwrap();
        assert_eq!(implementer.runtime.as_deref(), Some("hermes"));
        assert!(implementer.model.is_none());
        assert!(implementer.provider.is_none());
        assert_eq!(implementer.respond_to.as_deref(), Some("owner-only"));
        assert!(!implementer.env_vars.contains_key("INITIAL_AGENT_MODE"));
        assert!(implementer.system_prompt.contains("[IMPLEMENTATION_READY]"));
    }

    #[test]
    fn solo_dev_definitions_remain_editable_model_profiles() {
        let records = solo_dev_persona_records("2026-09-17T00:00:00Z").unwrap();
        for record in records {
            assert!(!record.is_builtin);
            assert!(record.source_team.is_none());
            assert!(record.source_team_persona_slug.is_none());
        }
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

    #[test]
    fn team_seed_is_idempotent_and_preserves_customization() {
        let mut teams = Vec::new();
        assert!(ensure_solo_dev_team_record(
            &mut teams,
            "2026-09-17T00:00:00Z"
        ));
        assert_eq!(teams.len(), 1);
        assert_eq!(teams[0].id, SOLO_DEV_TEAM_ID);
        assert_eq!(
            teams[0].persona_ids,
            vec![
                ARCHITECT_PERSONA_ID.to_string(),
                IMPLEMENTER_PERSONA_ID.to_string()
            ]
        );
        assert!(teams[0].is_builtin);

        teams[0].name = "My Solo Dev Team".to_string();
        teams[0].persona_ids = vec![ARCHITECT_PERSONA_ID.to_string()];
        assert!(!ensure_solo_dev_team_record(
            &mut teams,
            "2026-09-18T00:00:00Z"
        ));
        assert_eq!(teams[0].name, "My Solo Dev Team");
        assert_eq!(
            teams[0].persona_ids,
            vec![ARCHITECT_PERSONA_ID.to_string()]
        );
    }

    #[test]
    fn team_seed_repromotes_a_stale_marker_without_overwriting_fields() {
        let mut teams = Vec::new();
        ensure_solo_dev_team_record(&mut teams, "2026-09-17T00:00:00Z");
        teams[0].is_builtin = false;
        teams[0].name = "Customized".to_string();

        assert!(ensure_solo_dev_team_record(
            &mut teams,
            "2026-09-18T00:00:00Z"
        ));
        assert!(teams[0].is_builtin);
        assert_eq!(teams[0].name, "Customized");
        assert_eq!(teams[0].updated_at, "2026-09-18T00:00:00Z");
    }
}
