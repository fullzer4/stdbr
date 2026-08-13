# Changelog

## [0.0.3](https://github.com/fullzer4/stdbr/compare/v0.0.2...v0.0.3) (2026-08-13)


### Bug Fixes

* **release:** cross-compilation targets, wasm-pack action, python readme and pypi token guard ([efdeae9](https://github.com/fullzer4/stdbr/commit/efdeae9277acfcdd52f30ed23bfa9d1880b6b398))
* **release:** cross-compilation targets, wasm-pack action, python readme and pypi token guard ([da10ce7](https://github.com/fullzer4/stdbr/commit/da10ce7fa1dccfa14760ba9e51695c3e51ba9003))
* **release:** use env-based guard for PYPI_TOKEN instead of secrets in if ([b2c6741](https://github.com/fullzer4/stdbr/commit/b2c6741165f17bd43e09b2773001cc7a839ffaa2))

## [0.0.2](https://github.com/fullzer4/stdbr/compare/v0.0.1...v0.0.2) (2026-08-13)


### Features

* add cep, uf, and municipio modules with full cross-platform bin… ([2602b4f](https://github.com/fullzer4/stdbr/commit/2602b4f2d907671a4051f4312b1adc66e2d81974))
* add cep, uf, and municipio modules with full cross-platform bindings ([a44121d](https://github.com/fullzer4/stdbr/commit/a44121d0af0a7554070ada82451654156d156a2e))
* add parity tests for all bindings (Rust, Node.js, Python, FFI-C, WASM) ([c666267](https://github.com/fullzer4/stdbr/commit/c666267e05c53b461f68ebaa70be4d6f01c977b9))
* add Polars expression plugin for CPF, CNPJ, CEP ([f1fef84](https://github.com/fullzer4/stdbr/commit/f1fef848eb8921a359a58f6296ddade5d33923fb))
* add Polars expression plugin for CPF, CNPJ, CEP ([bec7335](https://github.com/fullzer4/stdbr/commit/bec73359383bb49fb5649f640d2232010a9f7dcd))
* add RG (Registro Geral) module for Brazilian identity validation ([75d428b](https://github.com/fullzer4/stdbr/commit/75d428bcfaa91f696793a69df0e6cd11d5769d6a))
* implement RG (Registro Geral) validation tests across multiple languages ([5f3795e](https://github.com/fullzer4/stdbr/commit/5f3795e7c9c09f16067ef30a8b73dcfaa51affdc))
* **rg:** consolidate generate, add masked/body, update all bindings ([3928c38](https://github.com/fullzer4/stdbr/commit/3928c38d2ee0c115dcafb76ae69387f8cd918d6d))
* **stdbr:** add root crate to enable release-please cargo-workspace ([d7831d3](https://github.com/fullzer4/stdbr/commit/d7831d32c58c9e5919f1136093aeefd34b6e4198))
* test release please alpha ([f5d8975](https://github.com/fullzer4/stdbr/commit/f5d89758f6b45badc9bc9b1c310b488d5f2c7545))


### Bug Fixes

* add .bazelignore, rename devshell test to check ([4f103e7](https://github.com/fullzer4/stdbr/commit/4f103e7695bf10d2175c6e0986729eb44c35af5a))
* add .bazelignore, rename test to check, fix license badge ([a06cde7](https://github.com/fullzer4/stdbr/commit/a06cde7388d182ba9462a2a805b9dc2340c7f759))
* add .bazelignore, rename test to check, fix license badge ([8e37fba](https://github.com/fullzer4/stdbr/commit/8e37fba0349e75a3f4497dd9fc42d8344e1fb857))
* add .bazelignore, rename test to check, fix license badge ([613da07](https://github.com/fullzer4/stdbr/commit/613da0779830a566233b261dc1c6df1cfb208580))
* apply cargo fmt and add .vscode config ([6904b6e](https://github.com/fullzer4/stdbr/commit/6904b6e4a3527746cef72c13a034f1c092f61db6))
* auto-fetch IBGE data in sync tests when missing ([f215eb1](https://github.com/fullzer4/stdbr/commit/f215eb170510e3a9c36a3e8893764fe0373fa5c7))
* **release-please:** correct trailing comma in extra-files config ([9d5b844](https://github.com/fullzer4/stdbr/commit/9d5b8445655b82895611d2a75044f9ddf929e6e0))
* **release-please:** remove redundant ffi-c extra-file to avoid tagged version conflict ([3023a0c](https://github.com/fullzer4/stdbr/commit/3023a0cdd9bb681631d2fdf77e200428db181d59))
* **release-please:** use literal version in workspace crates for cargo-workspace plugin ([7e6841a](https://github.com/fullzer4/stdbr/commit/7e6841a6fdca426c7ae45ceaab3870c048f1f2f1))
* **release:** add release-please dependency to publish-ffi job ([b09b6b2](https://github.com/fullzer4/stdbr/commit/b09b6b2700bbe5a5e7c45dd38264c800965f2617))
* replace genrule builds with native Bazel rules for nodejs, python, wasm ([d42d312](https://github.com/fullzer4/stdbr/commit/d42d31238d2189fa81c14f547d4d3a8190b32430))
* resolve clippy warnings and apply cargo fmt ([30101c4](https://github.com/fullzer4/stdbr/commit/30101c4191c540c591e3b5b5394e3345d93e4799))
* resolve clippy warnings and apply cargo fmt ([9ca48cb](https://github.com/fullzer4/stdbr/commit/9ca48cb913f035887d9c46dc4cab4d00ab87b066))
