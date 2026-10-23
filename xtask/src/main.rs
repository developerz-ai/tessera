//! `cargo xtask <guard>` — repo guards that a unit test cannot express. `bin/check` runs all.
//! Each guard prints one finding per line, naming the fix, and exits 1 when any fire.

mod boundaries;
mod links;
mod tiers;

use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

/// `CLAUDE.md` is read every turn; past this it is a token tax on every session.
const CLAUDE_MD_MAX_BYTES: u64 = 16 * 1024;

fn main() -> ExitCode {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..");
    let guard = std::env::args().nth(1).unwrap_or_else(|| "all".into());
    let findings = match guard.as_str() {
        "boundaries" => boundaries::run(&root),
        "links" => links::run(&root),
        "claude-md-size" => claude_md_size(&root),
        "all" => [
            boundaries::run(&root),
            links::run(&root),
            claude_md_size(&root),
        ]
        .concat(),
        other => vec![format!(
            "unknown guard `{other}`; one of: boundaries, links, claude-md-size, all"
        )],
    };
    let mut err = std::io::stderr().lock();
    for f in &findings {
        let _ = writeln!(err, "xtask {guard}: {f}");
    }
    if findings.is_empty() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

fn claude_md_size(root: &Path) -> Vec<String> {
    match std::fs::metadata(root.join("CLAUDE.md")) {
        Ok(m) if m.len() <= CLAUDE_MD_MAX_BYTES => vec![],
        Ok(m) => vec![format!(
            "CLAUDE.md is {} bytes (max {CLAUDE_MD_MAX_BYTES}); move detail into docs/ and link it",
            m.len()
        )],
        Err(e) => vec![format!("CLAUDE.md: {e}")],
    }
}
