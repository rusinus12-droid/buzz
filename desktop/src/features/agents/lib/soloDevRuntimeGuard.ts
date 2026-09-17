export const SOLO_DEV_TEAM_ID = "builtin-team:solo-dev";
export const UNCONFIGURED_RUNTIME_ID = "<unconfigured>";

type PersonaRuntimeRef = {
  runtime?: string | null;
};

type RuntimeRef = {
  id: string;
};

export type SoloDevRuntimeGuard = {
  strict: boolean;
  missingRuntimeIds: string[];
};

/**
 * Solo Dev is intentionally heterogeneous: Architect and Implementer must keep
 * the runtimes selected on their persona definitions. Generic Buzz teams may
 * fall back to a default runtime when one is unavailable, but doing that here
 * can silently turn both roles into the same agent runtime and defeats the
 * Codex ↔ Hermes collaboration contract.
 */
export function getSoloDevRuntimeGuard(
  teamId: string | null | undefined,
  personas: readonly PersonaRuntimeRef[],
  runtimes: readonly RuntimeRef[],
): SoloDevRuntimeGuard {
  if (teamId !== SOLO_DEV_TEAM_ID) {
    return { strict: false, missingRuntimeIds: [] };
  }

  const availableIds = new Set(runtimes.map((runtime) => runtime.id));
  const requiredIds = Array.from(
    new Set(
      personas.map((persona) => {
        const runtimeId = persona.runtime?.trim() ?? "";
        return runtimeId.length > 0 ? runtimeId : UNCONFIGURED_RUNTIME_ID;
      }),
    ),
  );

  return {
    strict: true,
    missingRuntimeIds: requiredIds.filter(
      (runtimeId) =>
        runtimeId === UNCONFIGURED_RUNTIME_ID || !availableIds.has(runtimeId),
    ),
  };
}
