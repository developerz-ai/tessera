# Docs

| Path | What | Read before |
|---|---|---|
| [spec/v0.1.md](spec/v0.1.md) | The language specification. Normative. | any change to syntax, types, semantics |
| [spec/packages.md](spec/packages.md) | Git-native packages; amends spec §8. | touching `tessera-pkg` |
| [spec/open-questions.md](spec/open-questions.md) | Known spec gaps, each an M0 issue. | guessing anything — don't |
| [architecture/crates.md](architecture/crates.md) | Crate map, tiers, frozen interfaces. | adding a dependency or a crate |
| [plans/](plans/README.md) | `/planx` output: dated, multi-file plans + `status.yml`. | starting a multi-PR change |

Style: lead with the rule, fragments over sentences, tables for any ≥3-row structure, date load-bearing claims (`As of 2026-10`). Delete stale docs rather than let them rot.
