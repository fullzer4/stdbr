const assert = require("node:assert/strict");
const { mkdtempSync, rmSync, writeFileSync } = require("node:fs");
const { tmpdir } = require("node:os");
const { join } = require("node:path");
const { spawnSync } = require("node:child_process");

const packageName = require("./package.json").name;
const npmCommand = process.platform === "win32" ? "npm.cmd" : "npm";
const temporaryDirectory = mkdtempSync(join(tmpdir(), "stdbr-nodejs-smoke-"));

function run(command, args, options = {}) {
  const result = spawnSync(command, args, {
    cwd: temporaryDirectory,
    encoding: "utf8",
    env: { ...process.env, npm_config_maxsockets: "4" },
    stdio: ["ignore", "pipe", "pipe"],
    ...options,
  });

  if (result.status !== 0) {
    throw new Error(
      `${command} ${args.join(" ")} failed\n${result.stdout}${result.stderr}`,
    );
  }

  return result.stdout;
}

try {
  const packOutput = run(
    npmCommand,
    ["pack", "--json", "--pack-destination", temporaryDirectory],
    { cwd: __dirname },
  );
  const [{ filename, files }] = JSON.parse(packOutput);
  const packedPaths = new Set(files.map(({ path }) => path));

  assert(packedPaths.has("index.js"), "npm pack must include index.js");
  assert(packedPaths.has("index.d.ts"), "npm pack must include index.d.ts");

  writeFileSync(
    join(temporaryDirectory, "package.json"),
    JSON.stringify({ private: true }),
  );
  run(npmCommand, [
    "install",
    "--ignore-scripts",
    "--no-audit",
    "--no-fund",
    join(temporaryDirectory, filename),
  ]);

  const commonJsOutput = run("node", [
    "--eval",
    `const stdbr = require(${JSON.stringify(packageName)}); process.stdout.write(String(stdbr.cpfIsValid("52998224725")))`,
  ]);
  assert.equal(commonJsOutput, "true");

  const esmOutput = run("node", [
    "--input-type=module",
    "--eval",
    `import stdbr, { cpfIsValid } from ${JSON.stringify(packageName)}; process.stdout.write(String(stdbr.cpfIsValid("52998224725") && cpfIsValid("52998224725")))`,
  ]);
  assert.equal(esmOutput, "true");
} finally {
  rmSync(temporaryDirectory, { recursive: true, force: true });
}
