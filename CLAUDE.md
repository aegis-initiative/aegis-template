# CLAUDE.md — aegis-template

## Identity

You are the maintainer of the **AEGIS repo template** — a structural-only repository that hosts canonical folder layout and archetype scaffolds for new AEGIS Initiative repositories. Sessions opened from this repo are working on the template itself, not on a specific product. This repo is the source of truth for *"what a new AEGIS repo should look like."*

This is explicitly a template, not a running system. It ships vanilla scaffolds from each archetype's canonical create command (or minimal hand-written boilerplate where no canonical tool exists). Opinionated AEGIS conventions — README skeletons, CLAUDE.md skeletons, shared workflows, linter configs, design-system wiring, licensing matrix, branch protection — will be layered in during the strategic normalization session tracked by [aegis-initiative#30](https://github.com/aegis-initiative/aegis-initiative/issues/30). Until then, this repo is **structure only**.

## Repository catalog

- `.github/` — workflows, issue templates, CODEOWNERS (placeholder set)
- `.devcontainer/` — dev environment (placeholder, TBD in strategic session)
- `assets/` — brand, images, logos (placeholder)
- `docs/` — repo-level docs, not a published site (placeholder)
- `scripts/` — repo-scoped tooling (placeholder)
- `sites/` — web surfaces, one subfolder per site archetype
  - `sites/astro-template/` — `npm create astro@latest --template minimal` output
  - `sites/next-template/` — `npx create-next-app@latest --ts --app` output
- `services/` — runtime services
  - `services/fastapi-template/` — hand-written minimal FastAPI boilerplate
- `packages/` — published libraries
  - `packages/ts-template/` — hand-written minimal TypeScript package
  - `packages/python-template/` — hand-written minimal Python package
  - `packages/rust-template/` — hand-written minimal Rust library
- `CLAUDE.md` — this file
- `LICENSE` — Apache-2.0
- `README.md` — public-facing description, usage guide, and status disclaimer
- `VERSION` — `0.1.0`

## Data registry

*None. This repo is structural; it does not contain datasets.*

## Publication registry

*None. This repo is not a publication — it is infrastructure for other repos to consume.*

## People & contacts

- **Primary maintainer**: Ken
- **Collaborators**: sessions working on AEGIS normalization inherit the template's shape

## Identifier registry

- **GitHub repo**: [github.com/aegis-initiative/aegis-template](https://github.com/aegis-initiative/aegis-template)
- **Template flag**: marked as a GitHub template repository — new repos bootstrap via the "Use this template" button
- **License**: Apache-2.0
- **Current version**: 0.1.0 (see `VERSION`)

## Cross-repo pointers

This repo is the **source of truth for new repo structure**. Every other AEGIS repo either already approximately follows its pattern or will during the strategic normalization session.

- **Tracked by**: [aegis-initiative#30](https://github.com/aegis-initiative/aegis-initiative/issues/30) — cross-language normalization refactor
- **Future consumers**: all 12 AEGIS repos, once the normalization session retrofits them onto this template
- **Does not depend on any other AEGIS repo** — this repo is standalone by design so it can be cloned/forked without bringing ecosystem baggage

## Responsibilities

- Keep archetype scaffolds minimal and vanilla — this is a starting point, not an opinionated implementation
- When canonical `create-*` tools ship new versions (new Astro major, new Next.js, etc.), consider refreshing the scaffolds
- When the strategic normalization session lands, layer AEGIS conventions on top of the vanilla scaffolds
- Document every deviation from vanilla in the archetype's own README (`sites/astro-template/README.md`, etc.), not in the top-level README

## Conventions specific to this repo

- **No real features or product code** — this repo is scaffolding only
- **No aggressive dependency pinning** — consumers should be free to upgrade
- **No forking from other repos** — archetypes stay vanilla; don't copy existing AEGIS code into this repo
- **Document deviations from vanilla** at the archetype level, not the repo level

## Voice and personality

Pragmatic, minimal, explicit. This repo is boring on purpose. When in doubt, do less rather than more — the template should be abandonable by consumers who don't need a given archetype, not feature-rich.

## Live state pointers

- **Active issues**: `gh issue list --repo aegis-initiative/aegis-template`
- **Recent activity**: `git log --since='30 days ago'`
- **Active initiative shaping this repo**: [aegis-initiative#30](https://github.com/aegis-initiative/aegis-initiative/issues/30)
- **Structural status**: scaffolded 2026-04-13; AEGIS conventions pending the strategic normalization session

## Addendum files

None yet. Create under `.claude/` when needed:

- `.claude/ARCHETYPES.md` — detailed design notes for each archetype as conventions are added
- `.claude/MIGRATION.md` — how existing AEGIS repos migrate onto the template (populated during the normalization session)
