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
- Spec §2 lexical status: `return` and `scope` are reserved; `in`, `or`, `key`, `index`, `bound`, `as`, `fail`, `fails`, `where`, `matching`, `unless` and `is` are contextual, each in its specified position. G4 wiring words and the S4 goal marker `idempotent` are classified; resource names such as `time` remain identifiers. Only the enumerated words of still-open G2 constructs remain unclassified ([#5](https://github.com/developerz-ai/tessera/issues/5)).
- Spec §2–§3 resolves G6 literal rules: contiguous `YYYY-MM-DD` takes priority over subtraction; invalid calendar dates and identifier suffixes are `E-LIT-001`, without fallback. Durations require an integer magnitude immediately followed by one of `ns us ms s m h d`, with an identifier boundary (`7d` is valid; `7days`, `5ms2` and `1.5d` are rejected). Text literals may contain spaces. An id literal `#12` requires a unique expected `Id<T>` or explicit `#12 : Id<Task>` ascription, a primary expression usable in calls and indexes; missing, ambiguous, non-ID or conflicting nominal context is `E-LIT-002`, with no default target or implicit conversion. These are normative rules and worked examples, not a lexer implementation ([#10](https://github.com/developerz-ai/tessera/issues/10)).
- Spec §2–§4: `view` is the single keyword for a pure derived value; the `fact` synonym is removed and is now an ordinary identifier ([#11](https://github.com/developerz-ai/tessera/issues/11)).
