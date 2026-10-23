//! Guard: internal dependencies only point DOWN a tier, every crate has a tier, and the
//! `CLAUDE.md` tier table matches `tiers.rs`.

use std::collections::BTreeSet;
use std::process::Command;

use serde_json::Value;

use crate::tiers::{TIERS, tier};

pub fn run(root: &std::path::Path) -> Vec<String> {
    let mut findings = Vec::new();
    let meta = match metadata(root) {
        Ok(m) => m,
        Err(e) => return vec![format!("cargo metadata failed: {e}")],
    };
    let members: BTreeSet<&str> = meta["packages"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|p| p["name"].as_str())
        .collect();
    for pkg in meta["packages"].as_array().into_iter().flatten() {
        let Some(name) = pkg["name"].as_str() else {
            continue;
        };
        if name == "xtask" {
            continue;
        }
        let Some(own) = tier(name) else {
            findings.push(format!(
                "{name}: no tier — add it to xtask/src/tiers.rs and the CLAUDE.md tier table"
            ));
            continue;
        };
        for dep in pkg["dependencies"].as_array().into_iter().flatten() {
            let Some(d) = dep["name"].as_str() else {
                continue;
            };
            if !members.contains(d) {
                continue;
            }
            match tier(d) {
                Some(t) if t < own => {}
                Some(t) => findings.push(format!(
                    "{name} (tier {own}) depends on {d} (tier {t}) — imports go DOWN only; \
                     move the shared part to a lower tier"
                )),
                None => findings.push(format!("{name} depends on untiered crate {d}")),
            }
        }
    }
    findings.extend(claude_md_agrees(root));
    findings
}

fn metadata(root: &std::path::Path) -> Result<Value, String> {
    let out = Command::new(std::env::var("CARGO").unwrap_or_else(|_| "cargo".into()))
        .args(["metadata", "--format-version", "1", "--no-deps"])
        .current_dir(root)
        .output()
        .map_err(|e| e.to_string())?;
    if !out.status.success() {
        return Err(String::from_utf8_lossy(&out.stderr).into_owned());
    }
    serde_json::from_slice(&out.stdout).map_err(|e| e.to_string())
}

/// Rows of the form ``| 3 | `tessera-interp`, `tessera-verify` | … |`` under `## Crate tiers`.
fn claude_md_agrees(root: &std::path::Path) -> Vec<String> {
    let Ok(text) = std::fs::read_to_string(root.join("CLAUDE.md")) else {
        return vec!["CLAUDE.md missing".into()];
    };
    let section = text.split("## Crate tiers").nth(1).unwrap_or("");
    let section = section.split("\n## ").next().unwrap_or("");
    let mut documented = BTreeSet::new();
    for line in section.lines() {
        let cells: Vec<&str> = line.split('|').map(str::trim).collect();
        let Some(t) = cells.get(1).and_then(|c| c.parse::<u8>().ok()) else {
            continue;
        };
        for name in cells.get(2).unwrap_or(&"").split('`').skip(1).step_by(2) {
            documented.insert((name.to_owned(), t));
        }
    }
    let actual: BTreeSet<(String, u8)> = TIERS.iter().map(|(n, t)| ((*n).to_owned(), *t)).collect();
    let mut f = Vec::new();
    for (n, t) in actual.difference(&documented) {
        f.push(format!(
            "CLAUDE.md tier table is missing `{n}` at tier {t} (tiers.rs has it)"
        ));
    }
    for (n, t) in documented.difference(&actual) {
        f.push(format!(
            "CLAUDE.md lists `{n}` at tier {t}, tiers.rs does not"
        ));
    }
    f
}
