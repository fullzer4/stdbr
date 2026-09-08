import assert from "node:assert/strict";
import { mkdtemp, readFile, readdir, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import { spawnSync } from "node:child_process";

const [artifactDirectory, expectedVersion = ""] = process.argv.slice(2);
if (!artifactDirectory) throw new Error("usage: smoke-wasm.mjs ARTIFACT_DIR [EXPECTED_VERSION]");

const archives = (await readdir(resolve(artifactDirectory))).filter((name) => name.endsWith(".tgz"));
assert.equal(archives.length, 1, "expected exactly one WASM tgz");
const temporaryDirectory = await mkdtemp(join(tmpdir(), "stdbr-wasm-smoke-"));

try {
  await writeFile(join(temporaryDirectory, "package.json"), JSON.stringify({ private: true, type: "module" }));
  const npm = process.platform === "win32" ? "npm.cmd" : "npm";
  const result = spawnSync(
    npm,
    ["install", "--ignore-scripts", "--no-audit", "--no-fund", "--no-package-lock", resolve(artifactDirectory, archives[0])],
    { cwd: temporaryDirectory, encoding: "utf8", env: { ...process.env, npm_config_cache: join(temporaryDirectory, ".npm-cache") } },
  );
  if (result.status !== 0) throw new Error(`${result.stdout}${result.stderr}`);

  const packageDirectory = join(temporaryDirectory, "node_modules", "@stdbr", "wasm");
  const manifest = JSON.parse(await readFile(join(packageDirectory, "package.json"), "utf8"));
  assert.equal(manifest.name, "@stdbr/wasm");
  await readFile(join(packageDirectory, "LICENSE"));
  if (expectedVersion) assert.equal(manifest.version, expectedVersion);

  const wasmFile = (await readdir(packageDirectory)).find((name) => name.endsWith("_bg.wasm"));
  assert.ok(wasmFile, "web package must contain its wasm binary");
  const module = await import(pathToFileURL(join(packageDirectory, manifest.module ?? manifest.main)).href);
  await module.default(await readFile(join(packageDirectory, wasmFile)));
  assert.equal(module.cpfIsValid("52998224725"), true);
} finally {
  await rm(temporaryDirectory, { recursive: true, force: true });
}
