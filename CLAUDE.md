# CLAUDE.md — aegis-template

## Role

This repo is a **structural template**, not a running service. A session opened from this repo is maintaining the archetype scaffolds and keeping the outer layout in sync with the conventions described in [aegis-initiative#30](https://github.com/aegis-initiative/aegis-initiative/issues/30).

## What to do here

- Keep archetype scaffolds minimal and vanilla — this is a starting point, not an opinionated implementation
- When `create-*` tools ship new versions, consider refreshing the scaffolds
- When the strategic normalization session lands, layer AEGIS conventions on top of the vanilla scaffolds
- Document every deviation from vanilla in the archetype's own README

## What NOT to do here

- Do not add real features or product code — this repo is scaffolding only
- Do not pin dependencies aggressively — consumers should be free to upgrade
- Do not fork from other repos and copy their code here — archetypes stay vanilla

## Strategic normalization is tracked at

https://github.com/aegis-initiative/aegis-initiative/issues/30
