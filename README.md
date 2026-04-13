# aegis-template

Canonical repo structure and archetype scaffolds for the AEGIS Initiative. Use this repo as a starting point when creating a new repository in the `aegis-initiative` org — click **"Use this template"** on GitHub, then delete the archetype folders you don't need.

## Status

**This repo is a structural template, not a normalized convention set.** The archetype scaffolds are whatever the canonical `create-*` commands (or minimal hand-written boilerplate) produce. Opinionated AEGIS conventions — CLAUDE.md content, README skeletons, workflow templates, design-system wiring, linter configs, branch protection, licensing matrix — will be layered in a dedicated session tracked by [aegis-initiative#30](https://github.com/aegis-initiative/aegis-initiative/issues/30). Until then, treat this repo as "structure only."

## Layout

```
.github/              # workflows, issue templates, CODEOWNERS
.devcontainer/        # dev environment (placeholder)
assets/               # brand, images, logos
docs/                 # repo-level docs (not a published site)
scripts/              # repo-scoped tooling
sites/                # web surfaces — one subfolder per site
  astro-template/     # vanilla Astro scaffold
  next-template/      # vanilla Next.js scaffold
services/             # runtime services — one subfolder per service
  fastapi-template/   # minimal FastAPI boilerplate
packages/             # published libraries — one subfolder per package
  ts-template/        # minimal TypeScript package
  python-template/    # minimal Python package
  rust-template/      # minimal Rust library
CLAUDE.md             # role instructions for the AI session that owns this repo
LICENSE               # Apache-2.0
README.md             # this file
VERSION               # committed version file
```

## How to use

1. On GitHub, click **"Use this template"** → **"Create a new repository"**.
2. Clone your new repo locally.
3. Delete archetype folders you don't need. Keep the ones that match your repo's purpose.
4. Rename the remaining archetype folders to match your project (e.g., `packages/ts-template/` → `packages/my-cool-lib/`).
5. Update `package.json`, `pyproject.toml`, `Cargo.toml` names to match.
6. Write your code.

## Archetypes

| Archetype | Folder | Source | Notes |
|---|---|---|---|
| Astro site | `sites/astro-template/` | `npm create astro@latest --template minimal` | For static content sites (docs, marketing, governance sites) |
| Next.js app | `sites/next-template/` | `npx create-next-app@latest --ts --app` | For interactive web apps (dashboards, platforms) |
| FastAPI service | `services/fastapi-template/` | Hand-written minimal boilerplate | For HTTP API services backed by Python |
| TypeScript package | `packages/ts-template/` | Hand-written minimal boilerplate | For npm-publishable TypeScript libraries |
| Python package | `packages/python-template/` | Hand-written minimal boilerplate | For PyPI-publishable Python libraries |
| Rust library | `packages/rust-template/` | Hand-written minimal boilerplate | For performance-critical native libraries |

## Non-archetype folders

Every repo — regardless of which archetypes it uses — should have:

- `.github/` — workflows, issue templates, CODEOWNERS (placeholder set here)
- `.devcontainer/` — dev environment (TBD in the strategic session)
- `assets/` — brand, images, logos (optional; delete if not needed)
- `docs/` — repo-level docs, not a published site
- `scripts/` — repo-scoped helper scripts
- `CLAUDE.md` — role instructions for the AI session that owns this repo
- `LICENSE`, `README.md`, `VERSION`

## What's not here yet

The strategic normalization session ([aegis-initiative#30](https://github.com/aegis-initiative/aegis-initiative/issues/30)) will add:

- AEGIS-flavored `README.md` skeleton (What / Why / Install / Use / Develop / License / Related)
- `CLAUDE.md` skeleton with a consistent structure across archetypes
- Canonical `.github/workflows/` (release pipeline, lint, link check, spellcheck)
- Shared linter configs (ESLint, Prettier, Ruff, rustfmt, etc.)
- `@aegis-initiative/design-system` pre-wired for the Astro + Next.js templates
- CalVer vs SemVer version conventions per archetype
- Dual-license matrix application (Apache-2.0 / BSL-1.1 / CC-BY-SA-4.0 / Proprietary)
- Branch protection + conventional commits enforcement
- `CODEOWNERS` template
- `dependabot.yml`
- `.devcontainer/` polyglot dev environment

## License

Apache-2.0. See [LICENSE](./LICENSE).
