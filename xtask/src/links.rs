//! Guard: every relative markdown link resolves to a file in the checkout.
//! Links inside fenced code blocks are examples, not links, and are skipped.

use std::path::{Path, PathBuf};

const SKIP_DIRS: &[&str] = &[".git", "target", "tmp", "node_modules"];

pub fn run(root: &Path) -> Vec<String> {
    let mut files = Vec::new();
    walk(root, &mut files);
    let mut findings = Vec::new();
    for file in files {
        let Ok(text) = std::fs::read_to_string(&file) else {
            continue;
        };
        let dir = file.parent().unwrap_or(root);
        let mut fenced = false;
        for (i, line) in text.lines().enumerate() {
            if line.trim_start().starts_with("```") {
                fenced = !fenced;
                continue;
            }
            if fenced {
                continue;
            }
            for target in targets(line) {
                let path = target.split('#').next().unwrap_or("");
                if path.is_empty() || dir.join(path).exists() {
                    continue;
                }
                let rel = file.strip_prefix(root).unwrap_or(&file).display();
                findings.push(format!("{rel}:{}: dead link -> {target}", i + 1));
            }
        }
    }
    findings
}

fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for e in entries.flatten() {
        let p = e.path();
        let name = e.file_name();
        if p.is_dir() {
            if !SKIP_DIRS.iter().any(|s| name == *s) {
                walk(&p, out);
            }
        } else if p.extension().is_some_and(|x| x == "md") {
            out.push(p);
        }
    }
}

/// Relative targets of `](target)` on one line; skips URLs, mailto and pure anchors.
fn targets(line: &str) -> Vec<&str> {
    let mut v = Vec::new();
    let mut rest = line;
    while let Some(i) = rest.find("](") {
        rest = &rest[i + 2..];
        let Some(end) = rest.find(')') else { break };
        let t = rest[..end].split_whitespace().next().unwrap_or("");
        if !(t.contains("://") || t.starts_with("mailto:") || t.starts_with('#')) {
            v.push(t);
        }
        rest = &rest[end..];
    }
    v
}

#[cfg(test)]
mod tests {
    use super::targets;

    #[test]
    fn extracts_only_relative_targets() {
        let l = "[a](docs/x.md#h) [b](https://x.y) [c](#top) [d](../e.md \"title\")";
        assert_eq!(targets(l), vec!["docs/x.md#h", "../e.md"]);
    }
}
