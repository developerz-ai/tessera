#!/usr/bin/env bash
# PostToolUse: rustfmt the .rs file an agent just wrote, so formatting is never a decision.
# Reads the hook payload on stdin; silent no-op for anything that isn't a Rust file.
set -uo pipefail
file=$(jq -r '.tool_input.file_path // empty' 2>/dev/null)
[[ "$file" == *.rs && -f "$file" ]] || exit 0
rustfmt --edition 2024 --config-path "$(git rev-parse --show-toplevel 2>/dev/null || pwd)/rustfmt.toml" "$file" 2>/dev/null || true
