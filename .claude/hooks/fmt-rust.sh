#!/usr/bin/env bash
# PostToolUse: rustfmt the .rs file an agent just wrote, so formatting is never a decision.
# Reads the hook payload on stdin; silent no-op for anything that isn't a Rust file.
set -uo pipefail
file=$(jq -r '.tool_input.file_path // empty' 2>/dev/null)
[[ "$file" == *.rs && -f "$file" ]] || exit 0
root=$(git rev-parse --show-toplevel 2>/dev/null || pwd)

# Format with the rustfmt from the toolchain pinned in rust-toolchain.toml: a shim
# earlier on PATH (mise, asdf, …) can pin another Rust, and the ambient rustfmt can
# write a shape the pinned `cargo fmt --check` then rejects. Same selection as bin/setup.
pinned=""
if [[ -f "$root/rust-toolchain.toml" ]]; then
  while IFS= read -r line || [[ -n "$line" ]]; do
    if [[ "$line" =~ ^[[:space:]]*channel[[:space:]]*=[[:space:]]*\"([^\"]+)\" ]]; then
      pinned="${BASH_REMATCH[1]}"
    fi
  done < "$root/rust-toolchain.toml"
fi
rustfmt_bin="rustfmt"
if [[ -n "$pinned" ]] && command -v rustup >/dev/null 2>&1; then
  pinned_rustfmt="$(rustup which --toolchain "$pinned" rustfmt 2>/dev/null)"
  if [[ -n "$pinned_rustfmt" ]]; then
    rustfmt_bin="$pinned_rustfmt"
  fi
fi

"$rustfmt_bin" --edition 2024 --config-path "$root/rustfmt.toml" "$file" 2>/dev/null || true
