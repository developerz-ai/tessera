# Changelog

All notable changes to this project are documented here. Format: [Keep a Changelog](https://keepachangelog.com/en/1.1.0/); versioning: [SemVer](https://semver.org/) once the first release ships.

## [Unreleased]

### Added
- Language specification v0.1 draft (`docs/spec/v0.1.md`) and git-native packages amendment (`docs/spec/packages.md`).
- Open-questions register (`docs/spec/open-questions.md`).
- Cargo workspace: one crate per work package, tier-enforced (`cargo xtask boundaries`).
- `tessera` CLI scaffold listing every spec command with its milestone; `--json` diagnostics.
- Gate (`bin/check`), CI, GitHub Pages site, agent setup (`CLAUDE.md`, `.claude/`).
