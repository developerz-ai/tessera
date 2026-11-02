# Changelog

All notable changes to this project are documented here. Format: [Keep a Changelog](https://keepachangelog.com/en/1.1.0/); versioning: [SemVer](https://semver.org/) once the first release ships.

## [Unreleased]

### Added
- Language specification v0.1 draft (`docs/spec/v0.1.md`) and git-native packages amendment (`docs/spec/packages.md`).
- Open-questions register (`docs/spec/open-questions.md`).
- Cargo workspace: one crate per work package, tier-enforced (`cargo xtask boundaries`).
- `tessera` CLI scaffold listing every spec command with its milestone; `--json` diagnostics.
- Gate (`bin/check`), CI, GitHub Pages site, agent setup (`CLAUDE.md`, `.claude/`).

### Changed
- Spec §2 lexical status: `return` and `scope` are reserved; `in`, `or`, `key`, `index`, `bound`, `as`, `fail`, `fails`, `where`, `matching`, `unless` and `is` are contextual, each with the one position where it is a keyword; the words §2 enumerates for constructs whose production is still open (G2, G4, S4) are not classified (15 of them were keywords in the previous §2 and are de-classified here), while `forall`, `comptime`, `is`, `in`, `scope` and `as` are classified although their production is still open ([#5](https://github.com/developerz-ai/tessera/issues/5)).
