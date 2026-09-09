import assert from "node:assert/strict";
import { mkdtemp, readFile, readdir, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import { spawnSync } from "node:child_process";

const [artifactDirectory, hostPlatform, expectedVersion = ""] = process.argv.slice(2);
if (!artifactDirectory || !hostPlatform) {
  throw new Error("usage: smoke-npm.mjs ARTIFACT_DIR HOST_PLATFORM [EXPECTED_VERSION]");
}

const expectedPlatforms = [
  "darwin-arm64",
  "darwin-x64",
  "linux-arm64-gnu",
  "linux-x64-gnu",
  "linux-x64-musl",
  "win32-x64-msvc",
];
const temporaryDirectory = await mkdtemp(join(tmpdir(), "stdbr-npm-smoke-"));
const npm = process.platform === "win32" ? "npm.cmd" : "npm";

try {
  const archives = (await readdir(resolve(artifactDirectory)))
    .filter((name) => name.endsWith(".tgz"))
    .map((name) => resolve(artifactDirectory, name));
  assert.equal(archives.length, 7, "expected the main tgz and six platform tgz files");

  await writeFile(join(temporaryDirectory, "package.json"), JSON.stringify({ private: true }));
  const result = spawnSync(
    npm,
    ["install", "--force", "--ignore-scripts", "--no-audit", "--no-fund", "--no-package-lock", ...archives],
    {
      cwd: temporaryDirectory,
      encoding: "utf8",
      env: { ...process.env, npm_config_cache: join(temporaryDirectory, ".npm-cache") },
      shell: process.platform === "win32",
    },
  );
  if (result.error) throw result.error;
  if (result.status !== 0) throw new Error(`${result.stdout}${result.stderr}`);

  const mainManifest = JSON.parse(
    await readFile(join(temporaryDirectory, "node_modules", "@stdbr", "stdbr", "package.json"), "utf8"),
  );
  await readFile(join(temporaryDirectory, "node_modules", "@stdbr", "stdbr", "LICENSE"));
  if (expectedVersion) assert.equal(mainManifest.version, expectedVersion);

  for (const platform of expectedPlatforms) {
    const packageDirectory = join(temporaryDirectory, "node_modules", "@stdbr", `stdbr-${platform}`);
    const manifest = JSON.parse(await readFile(join(packageDirectory, "package.json"), "utf8"));
    await readFile(join(packageDirectory, "LICENSE"));
    assert.equal(manifest.name, `@stdbr/stdbr-${platform}`);
    assert.equal(mainManifest.optionalDependencies[manifest.name], manifest.version);
    if (expectedVersion) assert.equal(manifest.version, expectedVersion);
    await readFile(join(packageDirectory, `stdbr.${platform}.node`));
  }

  const probe = `
    import { createRequire } from "node:module";
    import { join } from "node:path";
    import { pathToFileURL } from "node:url";
    const installDirectory = process.argv[1];
    const require = createRequire(join(installDirectory, "package.json"));
    const stdbr = require("@stdbr/stdbr");
    if (!stdbr.cpfIsValid("52998224725")) process.exit(1);
    const esm = await import(pathToFileURL(join(installDirectory, "node_modules", "@stdbr", "stdbr", "index.js")).href);
    if (!esm.cpfIsValid("52998224725")) process.exit(1);
  `;
  const probeResult = spawnSync(
    process.execPath,
    ["--input-type=module", "--eval", probe, temporaryDirectory],
    { encoding: "utf8" },
  );
  if (probeResult.error) throw probeResult.error;
  if (probeResult.status !== 0) throw new Error(`${probeResult.stdout}${probeResult.stderr}`);
  const libc = process.platform === "linux"
    ? process.report.getReport().header.glibcVersionRuntime ? "-gnu" : "-musl"
    : "";
  const toolchain = process.platform === "win32" ? "-msvc" : "";
  assert.equal(hostPlatform, `${process.platform}-${process.arch}${libc}${toolchain}`);
} finally {
  await rm(temporaryDirectory, { recursive: true, force: true });
}
