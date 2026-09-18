import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import test from "node:test";

test("team query refreshes personas after fork seed side effect", () => {
  const source = readFileSync(
    new URL("../teamHooks.ts", import.meta.url),
    "utf8",
  );

  assert.match(source, /const teams = await listTeams\(\);/);
  assert.match(
    source,
    /await queryClient\.invalidateQueries\(\{ queryKey: \["personas"\] \}\);/,
    "team loading must invalidate a persona cache that may have warmed before Solo Dev seeding",
  );
  assert.match(source, /return teams;/);
});
