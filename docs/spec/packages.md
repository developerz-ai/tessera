# Packages: git-native, with release tags

> Amends [v0.1 §8](v0.1.md#8-packages-hashing-and-loading). Status: **draft, pre-M0**. Replaces the central registry with git hosts; hashing, the content store, proof re-checking and loading in §8 are unchanged.

A dependency is a git URL plus a tag, with no central registry required. This works like Go modules, with the content hash added on top.

```toml
# tessera.toml
[deps]
auth   = { git = "github.com/acme/auth", version = "^2" }               # resolved from tags v2.x.y
sqlite = { git = "github.com/acme/sqlite", tag = "v4.1.0" }             # exact
http   = { git = "github.com/acme/mono", dir = "http", version = "^3" } # monorepo tag: http/v3.0.2
```

```text
# tessera.lock (committed)
auth  github.com/acme/auth  tag v2.1.0  commit 9fe3…  ir-hash blake3:41bc…
```

## Three identifiers

| Identifier | Property | Role |
|---|---|---|
| tag | movable, deletable | human-facing version |
| commit | immutable | what to fetch |
| IR hash | semantic | what the package *means* |

On every fetch the compiler checks out the commit, rebuilds the IR, and compares the hash. If the tag now points elsewhere, the build fails with **"tag moved"**, as with Go's checksum database.

## Release flow

```text
tessera release 2.2.0
  1. full check: all laws proven, examples pass, parity green
  2. compat diff vs previous tag: removed or changed signatures, weakened laws,
     or new capabilities need a major bump, otherwise release is blocked
  3. writes tessera.release.json (IR hash, capabilities, law and public-surface digest)
  4. creates a signed annotated tag v2.2.0 and pushes it
```

CI can run this on merge. No upload step exists, so the git host is the registry.

## Resolution and trust

| Topic | Rule |
|---|---|
| Version selection | Minimal version selection (lowest version satisfying all constraints). No SAT solver, so results are deterministic. Different majors coexist as different hashes, so conflicts are rare. |
| Signing | Signed tags (SSH, GPG or sigstore) and a trust policy in the manifest. `tessera audit` flags unsigned or unknown-signer dependencies. |
| Yanking | A `yanked` list in release metadata stops new resolves but keeps existing lockfiles working. |
| Fetch-time code | None. No build scripts, and `comptime` is pure. Fetching is the only network operation. |
| Prereleases | `v2.2.0-rc.1` tags are opt-in only. |

## Availability

Git hosts disappear. Because identity is the IR hash, any mirror can serve a package and the client verifies it.

- Global content-addressed cache in `~/.tessera/store`.
- Optional proxy (`TESSERA_PROXY`) that caches fetched packages.
- `tessera vendor` copies dependencies into the repo for air-gapped builds.
- Optional discovery index: a plain git repo mapping short names to URLs (like Homebrew taps), not required for resolving.

Open points this amendment raises → [open-questions.md](open-questions.md#packages).
