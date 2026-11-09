//! Layout and reachability invariants for `corpus/programs/`.
//!
//! Every directory under `corpus/programs/` is one example program: exactly one
//! `.tes` module plus one `README.md` recording, per issue anchor, whether that
//! program's syntax rests on that resolution. Nothing in the toolchain parses
//! `.tes` files yet, so without this suite those two files are review-enforced
//! only — and review is how the anchor rows and the declared failures drifted.
//!
//! The suite checks the four invariants that are decidable from the text:
//!
//! 1. exactly one `.tes` and one `README.md` per program directory;
//! 2. every `example <rule>(...) fails <Name>;` names a failure its rule can
//!    produce — the rule body carries a matching `or fail <Name>`
//!    (`../README.md#corpus-invariant-a-declared-failure-must-be-reachable`);
//! 3. the `#5`, `#7`, `#9` and `#10` anchor rows say YES exactly when the module
//!    uses that construct;
//! 4. the index's `Construct set` cell agrees with the program's own rows.
//!
//! `#6`, `#8` and `#15` are not derived: whether a program composes a
//! missing-production shape is a reading of the spec, not a text match, so those
//! rows stay a maintainer's call.
//!
//! The workspace lints deny `unwrap`/`expect`/`panic!` in test targets too, so
//! the helpers return `Result` and a test reports a broken file with `assert!`.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

/// The anchor rows derived from source: `#5` (`return` reserved), `#7` (the
/// statement separator), `#9` (`needs` grammar) and `#10` (literal typing).
const DERIVED_ANCHORS: [u8; 4] = [5, 7, 9, 10];

/// One example program: its directory name, module text and README text.
struct Program {
    name: String,
    source: String,
    readme: String,
}

/// `corpus/programs/`, found from the crate manifest directory so the suite does
/// not depend on the working directory.
fn programs_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../corpus/programs")
}

/// Read `path` as UTF-8 text, naming it in the error.
fn read(path: &Path) -> Result<String, String> {
    fs::read_to_string(path).map_err(|e| format!("cannot read {}: {e}", path.display()))
}

/// Every program directory, sorted by name so a failure names the same program
/// on every run (P1: same inputs, same outputs).
fn programs() -> Result<Vec<Program>, String> {
    let dir = programs_dir();
    let mut names = BTreeSet::new();
    for entry in fs::read_dir(&dir).map_err(|e| format!("cannot list {}: {e}", dir.display()))? {
        let entry = entry.map_err(|e| format!("cannot read an entry of {}: {e}", dir.display()))?;
        if entry.path().is_dir() {
            names.insert(entry.file_name().to_string_lossy().into_owned());
        }
    }
    assert!(!names.is_empty(), "corpus/programs/ must hold programs");

    let mut modules = Vec::new();
    for name in names {
        let program_dir = dir.join(&name);
        let mut found = Vec::new();
        for entry in fs::read_dir(&program_dir)
            .map_err(|e| format!("cannot list {}: {e}", program_dir.display()))?
        {
            let path = entry
                .map_err(|e| format!("cannot read an entry: {e}"))?
                .path();
            if path.extension().is_some_and(|ext| ext == "tes") {
                found.push(path);
            }
        }
        found.sort();
        assert_eq!(
            found.len(),
            1,
            "corpus/programs/{name}/ must hold exactly one `.tes` module, found {found:?}"
        );
        modules.push(Program {
            source: read(&found[0])?,
            readme: read(&program_dir.join("README.md"))
                .map_err(|e| format!("corpus/programs/{name}/ must hold a README.md: {e}"))?,
            name,
        });
    }
    Ok(modules)
}

/// The body of every `rule <name>(...) { ... }`, keyed by rule name. A body ends
/// at the `}` that returns the brace depth to zero, so a record or set literal
/// inside the body does not end it early.
fn rule_bodies(source: &str) -> BTreeMap<String, String> {
    let mut bodies = BTreeMap::new();
    let mut open: Option<(String, String, usize)> = None;
    for line in source.lines() {
        if let Some((name, body, mut depth)) = open.take() {
            let mut body = body;
            depth += count(line, '{');
            depth -= count(line, '}');
            body.push_str(line);
            body.push('\n');
            if depth == 0 {
                bodies.insert(name, body);
            } else {
                open = Some((name, body, depth));
            }
            continue;
        }
        let Some(rest) = line.strip_prefix("rule ") else {
            continue;
        };
        let Some((name, _)) = rest.split_once('(') else {
            continue;
        };
        let mut depth = count(line, '{');
        depth = depth.saturating_sub(count(line, '}'));
        let mut body = String::from(line);
        body.push('\n');
        if depth == 0 {
            // A one-line body: `rule f() { ... }`.
            bodies.insert(name.trim().to_owned(), body);
        } else {
            open = Some((name.trim().to_owned(), body, depth));
        }
    }
    bodies
}

/// How often `needle` occurs in `haystack`.
fn count(haystack: &str, needle: char) -> usize {
    haystack.chars().filter(|c| *c == needle).count()
}

/// Every `example <rule>(<args>) fails <Name>;` in a module, as
/// `(rule, failure, 1-based line)`.
fn failing_examples(source: &str) -> Vec<(String, String, usize)> {
    let mut examples = Vec::new();
    for (index, line) in source.lines().enumerate() {
        let Some(rest) = line.trim().strip_prefix("example ") else {
            continue;
        };
        let Some((call, tail)) = rest.split_once(')') else {
            continue;
        };
        let Some((rule, _)) = call.split_once('(') else {
            continue;
        };
        let Some(failure) = tail.trim().strip_prefix("fails ") else {
            continue;
        };
        examples.push((
            rule.trim().to_owned(),
            failure.trim().trim_end_matches(';').trim().to_owned(),
            index + 1,
        ));
    }
    examples
}

/// The `Used?` column of anchor row `#anchor`: `Some(true)` for `YES`,
/// `Some(false)` for `NONE`, `None` for a missing or malformed row.
fn anchor_row(readme: &str, anchor: u8) -> Option<bool> {
    let prefix = format!("#{anchor} ");
    readme
        .lines()
        .filter(|line| line.trim_start().starts_with('|'))
        .find_map(|line| {
            let cells: Vec<&str> = line.split('|').collect();
            (cells.len() >= 4 && cells[1].trim().starts_with(&prefix)).then_some(cells[3].trim())
        })
        .and_then(|used| {
            if used.starts_with("YES") {
                Some(true)
            } else if used.starts_with("NONE") {
                Some(false)
            } else {
                None
            }
        })
}

/// Whether the row for `anchor` is `YES` for this module, derived from source.
fn derived_row(anchor: u8, source: &str) -> bool {
    match anchor {
        // #5 — `return` is used as a reserved keyword at statement position.
        5 => source
            .lines()
            .any(|line| line.trim_start().starts_with("return ")),
        // #7 — the statement separator: every statement in these programs ends
        // with `;`, including the `or fail` forms the rows annotate.
        7 => source.lines().any(|line| line.trim_end().ends_with(';')),
        // #9 — the `needs` grammar declaring a capability.
        9 => source
            .lines()
            .any(|line| line.trim_start().starts_with("needs ")),
        // #10 — literal typing: an ascribed id literal (`#12 : Id<Task>`) or a
        // calendar `Date` literal (`2026-10-22`).
        10 => source
            .lines()
            .any(|line| is_literal_typing(strip_comment(line))),
        _ => false,
    }
}

/// `line` up to its first `//`, so a comment quoting a construct does not count
/// as a use of it.
fn strip_comment(line: &str) -> &str {
    line.split("//").next().unwrap_or(line)
}

/// Whether `line` carries an ascribed id literal or a calendar date literal.
fn is_literal_typing(line: &str) -> bool {
    let ascribed = line.split('#').skip(1).any(|tail| {
        let digits = tail.chars().take_while(char::is_ascii_digit).count();
        digits > 0 && tail[digits..].trim_start().starts_with(": Id<")
    });
    ascribed || has_calendar_date(line)
}

/// Whether `line` carries a `YYYY-MM-DD` calendar literal as a whole token.
fn has_calendar_date(line: &str) -> bool {
    let bytes = line.as_bytes();
    let digit = |at: usize| bytes.get(at).is_some_and(u8::is_ascii_digit);
    (0..bytes.len()).any(|at| {
        let head = at == 0 || !bytes[at - 1].is_ascii_digit();
        head && digit(at)
            && digit(at + 1)
            && digit(at + 2)
            && digit(at + 3)
            && bytes.get(at + 4) == Some(&b'-')
            && digit(at + 5)
            && digit(at + 6)
            && bytes.get(at + 7) == Some(&b'-')
            && digit(at + 8)
            && digit(at + 9)
            && !digit(at + 10)
    })
}

/// The index's `Construct set` column, per directory name.
fn index_construct_sets(index: &str) -> BTreeMap<String, BTreeSet<u8>> {
    let mut sets = BTreeMap::new();
    for line in index.lines() {
        let line = line.trim();
        if !line.starts_with("| [") {
            continue;
        }
        let cells: Vec<&str> = line.split('|').collect();
        if cells.len() < 4 {
            continue;
        }
        let Some(target) = cells[1].split('(').nth(1).and_then(|l| l.split(')').next()) else {
            continue;
        };
        let Some(name) = target.strip_suffix("/README.md") else {
            continue;
        };
        sets.insert(name.to_owned(), anchors_in(cells[3]));
    }
    sets
}

/// Every `#<n>` anchor named in a table cell.
fn anchors_in(cell: &str) -> BTreeSet<u8> {
    cell.split('#')
        .skip(1)
        .filter_map(|tail| {
            let digits: String = tail.chars().take_while(char::is_ascii_digit).collect();
            digits.parse().ok()
        })
        .collect()
}

/// One `.tes` module and one `README.md` per program directory.
#[test]
fn one_module_and_one_readme_per_program() -> Result<(), String> {
    for program in programs()? {
        assert!(
            program.source.starts_with("module "),
            "corpus/programs/{}/ module must open with its `module` header",
            program.name
        );
        assert!(
            program.readme.starts_with('#'),
            "corpus/programs/{}/README.md must be a Markdown document",
            program.name
        );
    }
    Ok(())
}

/// Every `example ... fails <Name>` names a failure its rule can reach, and no
/// guard that fails earlier on the empty store shadows it.
#[test]
fn every_declared_failure_is_reachable() -> Result<(), String> {
    let mut checked = 0;
    for program in programs()? {
        let bodies = rule_bodies(&program.source);
        for (rule, failure, line) in failing_examples(&program.source) {
            assert!(
                bodies.contains_key(&rule),
                "corpus/programs/{0}/: example on line {line} calls `{rule}`, which is not a rule in this module",
                program.name
            );
            let body = bodies.get(&rule).cloned().unwrap_or_default();
            let site = format!("or fail {failure}");
            let first = body.find(&site);
            assert!(
                first.is_some(),
                "corpus/programs/{0}/: example on line {line} declares `fails {failure}`, but rule `{rule}` has no `{site}` in its body — the failure is unreachable",
                program.name
            );
            let at = first.unwrap_or_default();
            let guard = described_guard(&body[..at]);
            assert!(
                !preceded_by_earlier_guard_failure(&body[..at]),
                "corpus/programs/{0}/: example on line {line} declares `fails {failure}`, but rule `{rule}` runs a guard that fails on the empty store before the `{site}` — the declared failure can never be the one a caller sees: {guard}",
                program.name
            );
            checked += 1;
        }
    }
    assert!(
        checked > 0,
        "corpus/programs/ must declare at least one failing example"
    );
    Ok(())
}

/// Whether `before` — the part of a rule body ahead of a `or fail <Name>` —
/// already contains a `require … or fail` guard. On the empty store every table
/// read misses, so such a guard fires before the later failure is reached.
fn preceded_by_earlier_guard_failure(before: &str) -> bool {
    before
        .lines()
        .any(|line| line.trim_start().starts_with("require ") && line.contains("or fail "))
}

/// The first guard that shadows a later failure, for the assertion message.
fn described_guard(before: &str) -> String {
    before
        .lines()
        .find(|line| line.trim_start().starts_with("require ") && line.contains("or fail "))
        .map(|line| line.trim().to_owned())
        .unwrap_or_default()
}

/// The `#5`, `#7`, `#9` and `#10` rows match what the module actually uses.
#[test]
fn anchor_rows_match_the_program_source() -> Result<(), String> {
    for program in programs()? {
        for anchor in DERIVED_ANCHORS {
            let row = anchor_row(&program.readme, anchor);
            assert!(
                row.is_some(),
                "corpus/programs/{0}/README.md must carry a `#{anchor}` anchor row whose `Used?` cell starts with YES or NONE",
                program.name
            );
            assert_eq!(
                row,
                Some(derived_row(anchor, &program.source)),
                "corpus/programs/{0}/README.md: the `#{anchor}` row disagrees with the program source",
                program.name
            );
        }
    }
    Ok(())
}

/// The index lists every program, and its `Construct set` cell agrees with the
/// program's own anchor rows.
#[test]
fn the_index_construct_set_agrees_with_the_program_rows() -> Result<(), String> {
    let index = read(&programs_dir().join("README.md"))?;
    let listed = index_construct_sets(&index);
    let programs = programs()?;
    assert_eq!(
        listed.keys().cloned().collect::<BTreeSet<_>>(),
        programs.iter().map(|p| p.name.clone()).collect(),
        "corpus/programs/README.md must list exactly the program directories"
    );

    for program in &programs {
        let index_set = &listed[&program.name];
        for anchor in DERIVED_ANCHORS {
            let used = derived_row(anchor, &program.source);
            // Both directions: an anchor the program uses must be listed, and an
            // anchor it does not use must not be — a construct set can over-claim
            // as easily as it can drift behind.
            assert_eq!(
                index_set.contains(&anchor),
                used,
                "corpus/programs/README.md: the `{}` row's construct set and the program source disagree about #{anchor}",
                program.name
            );
            assert_eq!(
                anchor_row(&program.readme, anchor),
                Some(used),
                "corpus/programs/README.md: the `{}` row's construct set says #{anchor}, but its README row disagrees",
                program.name
            );
        }
    }
    Ok(())
}
