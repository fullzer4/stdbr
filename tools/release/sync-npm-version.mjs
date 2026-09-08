import { readFile, writeFile } from "node:fs/promises";
import { join, resolve } from "node:path";

const sourceDirectory = resolve(process.argv[2] ?? "");
if (!process.argv[2]) {
  throw new Error("usage: sync-npm-version.mjs NODE_PACKAGE_DIR");
}

async function readJson(path) {
  return JSON.parse(await readFile(path, "utf8"));
}

async function writeJson(path, value) {
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`);
}

const mainManifestPath = join(sourceDirectory, "package.json");
const mainManifest = await readJson(mainManifestPath);
const version = mainManifest.version;
if (!/^\d+\.\d+\.\d+(?:-[0-9A-Za-z.-]+)?$/.test(version)) {
  throw new Error(`invalid package version: ${version}`);
}

for (const dependency of Object.keys(mainManifest.optionalDependencies ?? {})) {
  mainManifest.optionalDependencies[dependency] = version;
}
await writeJson(mainManifestPath, mainManifest);

for (const platform of [
  "darwin-arm64",
  "darwin-x64",
  "linux-arm64-gnu",
  "linux-x64-gnu",
  "linux-x64-musl",
  "win32-x64-msvc",
]) {
  const manifestPath = join(sourceDirectory, "npm", platform, "package.json");
  const manifest = await readJson(manifestPath);
  manifest.version = version;
  await writeJson(manifestPath, manifest);
}
