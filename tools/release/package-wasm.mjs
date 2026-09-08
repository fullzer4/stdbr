import { copyFile, mkdir, readFile, writeFile } from "node:fs/promises";
import { dirname, resolve } from "node:path";
import { spawnSync } from "node:child_process";

const [generatedDirectory, sourceManifestPath, outputDirectory] = process.argv.slice(2);
if (!generatedDirectory || !sourceManifestPath || !outputDirectory) {
  throw new Error("usage: package-wasm.mjs GENERATED_DIR SOURCE_PACKAGE_JSON OUTPUT_DIR");
}

const generatedManifestPath = resolve(generatedDirectory, "package.json");
const generated = JSON.parse(await readFile(generatedManifestPath, "utf8"));
const source = JSON.parse(await readFile(sourceManifestPath, "utf8"));

generated.name = source.name;
generated.version = source.version;
generated.license = source.license;
generated.publishConfig = source.publishConfig;

await writeFile(generatedManifestPath, `${JSON.stringify(generated, null, 2)}\n`);
await copyFile(resolve(dirname(sourceManifestPath), "../../LICENSE"), resolve(generatedDirectory, "LICENSE"));
await mkdir(outputDirectory, { recursive: true });

const result = spawnSync(
  process.platform === "win32" ? "npm.cmd" : "npm",
  ["pack", resolve(generatedDirectory), "--ignore-scripts", "--pack-destination", resolve(outputDirectory)],
  { stdio: "inherit" },
);
if (result.status !== 0) process.exit(result.status ?? 1);
