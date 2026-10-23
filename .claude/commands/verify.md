---
description: Run the gate (bin/check) and fix what it reports
---

Run `bin/check`.

If it fails: fix every reported failure, re-run until green. One line per fix. Never narrow the gate, never `#[allow]` a lint or skip a test to get green — fix the code. A guard finding (`xtask …:`) names its fix; do that first.

If a failure exposes a spec ambiguity rather than a code bug, stop and run `/spec-issue` instead of guessing.
