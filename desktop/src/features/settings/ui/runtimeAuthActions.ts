import type { AcpAuthMethod, AcpRuntimeCatalogEntry } from "@/shared/api/types";

export const HERMES_SETUP_AUTH_METHOD_ID = "hermes-setup";

/**
 * Most runtimes only need auth actions when discovery says they are logged
 * out. Hermes is different: its ACP adapter always advertises a terminal
 * provider/model setup action, including when credentials already exist. Keep
 * that action reachable from Buzz so Ollama Cloud (or another provider) can be
 * configured/reconfigured without asking the user to launch Hermes manually.
 */
export function shouldDiscoverRuntimeAuthActions(
  runtime: Pick<AcpRuntimeCatalogEntry, "id" | "availability" | "authStatus">,
): boolean {
  return (
    runtime.availability === "available" &&
    (runtime.authStatus.status === "logged_out" || runtime.id === "hermes")
  );
}

/**
 * For Hermes expose only the adapter's explicit terminal setup action. Provider
 * credential rows are useful to generic ACP clients but add noise here: the
 * setup wizard is the one action that can add or change Ollama Cloud/provider
 * credentials and model selection.
 */
export function visibleRuntimeAuthActions(
  runtimeId: string,
  methods: readonly AcpAuthMethod[],
): AcpAuthMethod[] {
  if (runtimeId !== "hermes") {
    return [...methods];
  }
  return methods.filter((method) => method.id === HERMES_SETUP_AUTH_METHOD_ID);
}
