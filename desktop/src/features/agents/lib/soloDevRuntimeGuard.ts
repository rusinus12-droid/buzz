export const SOLO_DEV_TEAM_ID = "builtin-team:solo-dev";
export const SOLO_DEV_ARCHITECT_PERSONA_ID = "solo-dev:architect";
export const SOLO_DEV_IMPLEMENTER_PERSONA_ID = "solo-dev:implementer";

type PersonaRuntimeRef = {
  id?: string | null;
  runtime?: string | null;
};

type RuntimeRef = {
  id: string;
};

export type SoloDevRuntimeGuard = {
  strict: boolean;
  missingRuntimeIds: string[];
  roleContractErrors: string[];
};

const EXPECTED_ROLES = [
  {
    personaId: SOLO_DEV_ARCHITECT_PERSONA_ID,
    label: "Architect",
    runtimeId: "codex",
  },
  {
    personaId: SOLO_DEV_IMPLEMENTER_PERSONA_ID,
    label: "Implementer",
    runtimeId: "hermes",
  },
] as const;

/**
 * Solo Dev is intentionally heterogeneous and has an exact two-role contract:
 * Architect must be Codex and Implementer must be Hermes. Generic Buzz teams
 * may fall back to a default runtime when one is unavailable, but doing that
 * here can silently collapse both roles onto one harness or deploy only one
 * side of the collaboration loop.
 */
export function getSoloDevRuntimeGuard(
  teamId: string | null | undefined,
  personas: readonly PersonaRuntimeRef[],
  runtimes: readonly RuntimeRef[],
): SoloDevRuntimeGuard {
  if (teamId !== SOLO_DEV_TEAM_ID) {
    return { strict: false, missingRuntimeIds: [], roleContractErrors: [] };
  }

  const availableIds = new Set(runtimes.map((runtime) => runtime.id));
  const personasById = new Map(
    personas
      .filter((persona) => persona.id)
      .map((persona) => [persona.id as string, persona]),
  );

  const roleContractErrors: string[] = [];

  if (personas.length !== EXPECTED_ROLES.length) {
    roleContractErrors.push(
      `Solo Dev requires exactly ${EXPECTED_ROLES.length} roles (Architect and Implementer), but ${personas.length} resolved.`,
    );
  }

  for (const expected of EXPECTED_ROLES) {
    const persona = personasById.get(expected.personaId);
    if (!persona) {
      roleContractErrors.push(`${expected.label} role is missing from the team.`);
      continue;
    }

    const configuredRuntime = persona.runtime?.trim() ?? "";
    if (configuredRuntime !== expected.runtimeId) {
      roleContractErrors.push(
        `${expected.label} must use ${expected.runtimeId}; configured runtime is ${configuredRuntime || "(unset)"}.`,
      );
    }
  }

  return {
    strict: true,
    missingRuntimeIds: EXPECTED_ROLES.map((role) => role.runtimeId).filter(
      (runtimeId, index, all) =>
        all.indexOf(runtimeId) === index && !availableIds.has(runtimeId),
    ),
    roleContractErrors,
  };
}
