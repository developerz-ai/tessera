# Security Policy

## Supported versions

Tessera is pre-release (pre-M0). Only `main` receives fixes.

## Reporting a vulnerability

**Do not open a public issue.** Report privately via [GitHub security advisories](https://github.com/developerz-ai/tessera/security/advisories/new). Expect an acknowledgement within 7 days.

## What counts

Beyond memory-safety and supply-chain issues, these are security bugs in a verifying compiler:

| Class | Example |
|---|---|
| **False proof** | a law reported Proven that has a counterexample; Unknown treated as success |
| **Capability escape** | code using an effect it didn't declare in `needs`, or a dependency exceeding its `[grant]` |
| **Secret leak** | a `Secret<T>` reaching a log, output, surface response or non-`Secret` store column |
| **Hash confusion** | two semantically different programs with one content hash; a lockfile accepting a moved tag |
| **Fetch-time execution** | any code running while a dependency is fetched |
| **Nondeterminism** | interpreter and a backend producing different results for one input log |
