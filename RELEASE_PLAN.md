# Plano de Deploy e Versionamento — stdbr

> Estratégia baseada em **release-please**, **Semantic Versioning**, **Conventional Commits** e publicação automatizada em múltiplos registries.

---

## 1. Visão geral

O `stdbr` é uma biblioteca multi-linguagem com um único núcleo em Rust (`stdbr-core`) e bindings para Node.js, Python, WASM e C/C++. Todos os artefatos compartilham a **mesma versão** (lockstep), garantindo que o mesmo comportamento esteja disponível em todas as plataformas.

### Ferramentas principais

| Responsabilidade | Ferramenta |
|------------------|------------|
| Versionamento e changelog | `release-please` |
| Commits | Conventional Commits |
| Publicação Rust | `cargo publish` |
| Publicação Node.js | `napi-rs` + `npm publish` |
| Publicação Python | `maturin` + `maturin publish` |
| Publicação WASM | `wasm-pack` + `npm publish` |
| Publicação C/C++ | GitHub Releases + futuro Conan/vcpkg |

### Fluxo de release

```text
push/merge na main
        ↓
release-please abre/atualiza PR: "chore: release v0.2.0"
        ↓
PR contém:
  - CHANGELOG.md
  - bump em Cargo.toml (workspace + crates)
  - bump em bindings/nodejs/package.json
  - bump em bindings/wasm/package.json
  - bump em bindings/python/pyproject.toml
  - bump em todos os bindings/nodejs/npm/*/package.json
        ↓
mantenedor mergeia o PR
        ↓
release-please cria:
  - git tag v0.2.0
  - GitHub Release v0.2.0
        ↓
workflow publish.yml dispara e publica:
  - crates.io
  - npm (@stdbr/stdbr + @stdbr/wasm)
  - PyPI
  - GitHub Release artifacts (C/C++)
```

---

## 2. Estratégia de versionamento

### Lockstep SemVer

Todos os pacotes compartilham a mesma versão:

| Pacote | Registry | Versão |
|--------|----------|--------|
| `stdbr-core` | crates.io | `x.y.z` |
| `@stdbr/stdbr` | npm | `x.y.z` |
| `stdbr` (Python) | PyPI | `x.y.z` |
| `@stdbr/wasm` | npm | `x.y.z` |
| `stdbr-ffi` | GitHub Releases | `x.y.z` |

### Versionamento automático com release-please

- `fix:` → bump patch (`0.0.1` → `0.0.2`)
- `feat:` → bump minor (`0.0.1` → `0.1.0`)
- `feat!:`, `fix!:` ou `BREAKING CHANGE:` → bump major (`0.0.1` → `1.0.0`)
- Versão 0.x: `bump-minor-pre-major` e `bump-patch-for-minor-pre-major` ativados.

---

## 3. Arquivos de configuração do release-please

### `release-please-config.json`

Configura um único package na raiz do tipo `rust` e usa `extra-files` para atualizar os manifests das outras linguagens em lockstep.

### `.release-please-manifest.json`

Registra a versão atual do pacote raiz. O release-please o atualiza automaticamente a cada release.

---

## 4. Branches e pre-releases

| Branch | Configuração | Exemplo de versão |
|--------|--------------|-------------------|
| `main` | release normal | `0.2.0` |
| `next` | `prerelease: true`, `prerelease-type: "next"` | `0.2.0-next.1` |
| `beta` | `prerelease: true`, `prerelease-type: "beta"` | `0.2.0-beta.1` |
| `alpha` | `prerelease: true`, `prerelease-type: "alpha"` | `0.2.0-alpha.1` |

### Estratégia recomendada para pre-releases

- Desenvolver novas funcionalidades em PRs para `main`.
- Para testar publicações antes do release oficial, mergear `main` em `alpha`/`beta`/`next`.
- Quando estiver pronto, mergear `next` → `main` para gerar o release final.

---

## 5. Workflows do GitHub Actions

### 5.1 Criar Release PR — `.github/workflows/release-please.yml`

Disparado em `push` para `main`, `next`, `beta` e `alpha`. Usa um PAT (`RELEASE_PLEASE_TOKEN`) para que o merge do release PR dispare o workflow de publicação.

### 5.2 Publicar nos registries — `.github/workflows/publish.yml`

Disparado quando o release-please cria um GitHub Release. Contém os jobs:

1. `publish-crates` → publica `stdbr-core` no crates.io.
2. `build-nodejs` → builda bins nativos multi-plataforma (Linux glibc/musl, macOS x64/ARM64, Windows x64).
3. `publish-nodejs` → publica pacotes `@stdbr/stdbr-*` e o pacote principal `@stdbr/stdbr`.
4. `publish-wasm` → builda com `wasm-pack` e publica `@stdbr/wasm`.
5. `publish-python` → builda e publica wheels multi-plataforma no PyPI via `maturin-action`.
6. `build-ffi` + `publish-ffi` → builda bibliotecas C/C++ e anexa tarballs ao GitHub Release.

---

## 6. Ajustes nos arquivos do projeto

### `Cargo.toml` do workspace

Metadados de publicação herdados por todos os crates: `description`, `repository`, `homepage`, `readme`, `keywords`, `categories`.

### Crates do workspace

Cada `Cargo.toml` de crate publicável herda os metadados do workspace via `.workspace = true`.

### `bindings/nodejs/package.json`

Adicionados:
- `publishConfig.access: public`
- `scripts.build`, `scripts.artifacts`, `scripts.prepublishOnly`
- `optionalDependencies` apontando para pacotes por plataforma

### Pacotes por plataforma `bindings/nodejs/npm/*/package.json`

Todos são incluídos como `extra-files` no release-please para manter a mesma versão em lockstep.

### `bindings/wasm/package.json`

Adicionados:
- `publishConfig.access: public`
- `files: ["pkg"]`

### `bindings/python/pyproject.toml`

- `version` passou de `dynamic` para estático (`0.0.1`).
- Adicionados `description`, `readme`, `license`, `classifiers`.

---

## 7. Commit convention

Adotar **Conventional Commits** obrigatoriamente. A mensagem do squash-merge do PR deve seguir o padrão.

### Exemplos

```text
fix(cpf): reject all-equal digit sequences
feat(cep): add state lookup by IBGE code
feat(cnpj)!: change alphanumeric API surface
chore(deps): update napi-rs to v3
```

### Recomendações

- Sempre fazer **squash-merge** na `main`.
- A mensagem final do squash deve descrever o impacto da mudança.
- Evitar commits `feat` dentro de PRs que não devem gerar release (use `chore` ou `refactor`).

---

## 8. Segurança e tokens necessários

| Secret | Onde usar | Tipo |
|--------|-----------|------|
| `RELEASE_PLEASE_TOKEN` | workflow release-please | GitHub PAT |
| `CARGO_REGISTRY_TOKEN` | `cargo publish` | crates.io API token |
| `NPM_TOKEN` | `npm publish` | npm access token ou OIDC |
| `PYPI_TOKEN` | `maturin publish` | PyPI API token ou OIDC |

### Recomendações de segurança

1. **Trusted Publishing (OIDC)** no npm e PyPI, eliminando tokens longos.
2. Para crates.io, usar token de publicação com escopo limitado.
3. O PAT do release-please precisa de permissões `contents:write`, `pull-requests:write` e `workflows:write`.
4. Habilitar branch protection na `main` para exigir PR review antes do merge.

---

## 9. IBGE Sync e releases automáticos

O workflow `ibge-sync.yml` foi ajustado para **abrir um Pull Request** em vez de uma issue quando os dados do IBGE estiverem desatualizados.

Quando esse PR for mergeado com mensagem `chore(ibge): ...`, ele não gera release por si só. Mas se vier acompanhado de `feat` ou `fix`, o release-please incluirá a mudança no próximo release.

---

## 10. Checklist de implementação

| # | Tarefa | Status |
|---|--------|--------|
| 1 | Criar `release-please-config.json` | ✅ |
| 2 | Criar `.release-please-manifest.json` | ✅ |
| 3 | Criar `.github/workflows/release-please.yml` | ✅ |
| 4 | Criar `.github/workflows/publish.yml` | ✅ |
| 5 | Adicionar metadados de publicação nos `Cargo.toml` | ✅ |
| 6 | Adicionar `version` e scripts em `bindings/nodejs/package.json` | ✅ |
| 7 | Adicionar `version` e `publishConfig` em `bindings/wasm/package.json` | ✅ |
| 8 | Adicionar `version` e metadados em `bindings/python/pyproject.toml` | ✅ |
| 9 | Configurar secrets `RELEASE_PLEASE_TOKEN`, `CARGO_REGISTRY_TOKEN`, `NPM_TOKEN`, `PYPI_TOKEN` | ⬜ |
| 10 | Configurar trusted publishing no npm e PyPI | ⬜ |
| 11 | Documentar Conventional Commits em `CONTRIBUTING.md` | ✅ |
| 12 | Ajustar `ibge-sync.yml` para abrir PR ao invés de issue | ✅ |
| 13 | Testar com release `0.1.0-alpha.1` na branch `alpha` | ⬜ |

---

## 11. Primeiro teste recomendado

1. Criar branch `alpha` a partir da `main`.
2. Configurar `release-please-config.json` com `prerelease: true` e `prerelease-type: "alpha"`.
3. Fazer um commit `feat: initial release setup` na `alpha`.
4. Verificar se o release-please abre o PR `chore: release v0.1.0-alpha.1`.
5. Mergear o PR e verificar se o workflow `publish.yml` executa.
6. Validar a publicação em todos os registries.

---

## 12. Considerações futuras

- **Conan/vcpkg para C/C++:** após estabilizar o GitHub Releases, avaliar publicar em:
  - [ConanCenter](https://conan.io/center/)
  - [vcpkg](https://vcpkg.io/)
- **Badge de versão:** adicionar badges de versão no `README.md` para cada registry.
- **Documentação automática:** gerar docs a partir do release-please e publicar em GitHub Pages.
