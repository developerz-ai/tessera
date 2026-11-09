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
//!    uses that construct — `#7` from a rule body carrying two or more
//!    `;`-terminated statements, since every module's `module …;` header ends
//!    with `;` and would satisfy any weaker test;
//! 4. the index's `Construct set` cell agrees with the program's own rows, and
//!    the resolution links live in the index alone, not copied into every
//!    program README;
//! 5. every `:N` citation inside an anchor row names a real line of the module,
//!    and a `YES` row for a derived anchor cites every evidence line — `#5`,
//!    `#7`, `#9` and `#10` are exact against the source, so a citation that
//!    rotated with an edit fails here instead of waiting for review.
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

/// Every anchor row a program README carries. `#6`, `#8` and `#15` have no
/// derived evidence set, but the citations they do carry must still name code.
const ALL_ANCHORS: [u8; 7] = [5, 6, 7, 8, 9, 10, 15];

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

/// How many `;`-terminated statements a rule body carries, comments and string
/// literals ignored.
///
/// Counting lines instead would under-count a body that writes two statements on
/// one line (`let t = tasks[id] or fail NotFound; require t.owner == by or fail
/// Forbidden;` is two statements, one line), and counting raw `;` would count one
/// inside a `"…"` literal.
fn statements(body: &str) -> usize {
    let mut count = 0;
    let mut in_literal = false;
    let mut escaped = false;
    for ch in body.lines().map(strip_comment).collect::<String>().chars() {
        if in_literal {
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                in_literal = false;
            }
        } else if ch == '"' {
            in_literal = true;
        } else if ch == ';' {
            count += 1;
        }
    }
    count
}

/// `statements` counts `;`-terminated statements, not lines that end in `;`.
///
/// The line-counting version returned 1 for the first case below — one line, two
/// statements — so a rule body written compactly derived `#7` as `NONE` while
/// the source plainly uses the separator. A `;` inside a string literal is not a
/// statement, and a trailing comment is not either.
#[test]
fn statements_counts_statements_not_lines() {
    assert_eq!(
        statements("  let a = 1 or fail X; let b = 2 or fail Y;\n"),
        2
    );
    assert_eq!(
        statements("  let a = 1 or fail X;\n  let b = 2 or fail Y;\n"),
        2
    );
    assert_eq!(statements("  require t.name != \"a;b\" or fail X;\n"), 1);
    assert_eq!(statements("  let a = 1 or fail X; // and; not; this\n"), 1);
    assert_eq!(statements("  return t;\n"), 1);
    assert_eq!(statements("  // only prose; here\n"), 0);
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

/// The 1-based README line and `Used?` cell of anchor row `#anchor`, or `None`
/// when the row is missing or malformed.
///
/// A row is exactly `| <issue> | <used?> |`: the resolution link lives in the
/// index and nowhere else, so a row that grows a link column again is malformed
/// and fails here rather than drifting into twenty stale copies.
fn anchor_cell(readme: &str, anchor: u8) -> Option<(usize, String)> {
    let prefix = format!("#{anchor} ");
    readme.lines().enumerate().find_map(|(index, line)| {
        if !line.trim_start().starts_with('|') {
            return None;
        }
        let cells: Vec<&str> = line.split('|').collect();
        (cells.len() == 4 && cells[1].trim().starts_with(&prefix))
            .then(|| (index + 1, cells[2].trim().to_owned()))
    })
}

/// The `Used?` column of anchor row `#anchor`: `Some(true)` for `YES`,
/// `Some(false)` for `NONE`, `None` for a missing or malformed row.
fn anchor_row(readme: &str, anchor: u8) -> Option<bool> {
    let (_, used) = anchor_cell(readme, anchor)?;
    if used.starts_with("YES") {
        Some(true)
    } else if used.starts_with("NONE") {
        Some(false)
    } else {
        None
    }
}

/// Whether the row for `anchor` is `YES` for this module, derived from source.
fn derived_row(anchor: u8, source: &str) -> bool {
    match anchor {
        // #5 — `return` is used as a reserved keyword at statement position.
        5 => source
            .lines()
            .any(|line| line.trim_start().starts_with("return ")),
        // #7 — the statement separator: a rule body carries two or more
        // `;`-terminated statements, including the `or fail` forms the rows
        // annotate. The `module …;` header ends with `;` in every module, so on
        // its own it is evidence of nothing.
        7 => rule_bodies(source)
            .values()
            .any(|body| statements(body) >= 2),
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

/// The 1-based lines of every `rule` declaration and its body, by brace depth:
/// the declaration line plus the lines up to the `}` that returns depth to zero.
fn rule_body_lines(source: &str) -> BTreeSet<usize> {
    let mut lines = BTreeSet::new();
    let mut depth = 0usize;
    let mut inside = false;
    for (index, line) in source.lines().enumerate() {
        if !inside {
            if !(line.starts_with("rule ") && line.contains('(')) {
                continue;
            }
            inside = true;
            depth = 0;
        }
        lines.insert(index + 1);
        depth = depth
            .saturating_add(count(line, '{'))
            .saturating_sub(count(line, '}'));
        if depth == 0 {
            inside = false;
        }
    }
    lines
}

/// The 1-based module lines that are evidence for `anchor` — the lines a `YES`
/// row must cite: a `return` at statement position (`#5`), an `or fail` site in
/// a rule body (`#7`), a `needs` clause (`#9`), an ascribed id literal or a
/// calendar `Date` literal (`#10`), or an `example` call (`#15`).
///
/// `#6` and `#8` stay empty: whether a program composes a missing-production
/// shape is a maintainer's reading of the spec, not a text match. `#15`'s claim —
/// every `example` argument is a declared `fixture` — is text-decidable, which is
/// what `example_arguments_are_declared_fixtures` pins.
fn evidence_lines(anchor: u8, source: &str) -> BTreeSet<usize> {
    let body = rule_body_lines(source);
    source
        .lines()
        .enumerate()
        .filter(|(index, line)| match anchor {
            5 => line.trim_start().starts_with("return "),
            7 => body.contains(&(index + 1)) && strip_comment(line).contains("or fail "),
            9 => line.trim_start().starts_with("needs "),
            10 => is_literal_typing(strip_comment(line)),
            15 => line.trim_start().starts_with("example "),
            _ => false,
        })
        .map(|(index, _)| index + 1)
        .collect()
}

/// Every `:N` line citation in `text`. A `:` whose preceding character is
/// alphanumeric is not a citation: `#12 : Id<Task>` is an ascription and
/// `docs/spec/v0.1.md:640` is a spec link, not a line of this module.
fn cited_lines(text: &str) -> BTreeSet<usize> {
    let bytes = text.as_bytes();
    let mut cited = BTreeSet::new();
    for at in 0..bytes.len() {
        if bytes[at] != b':' || (at > 0 && bytes[at - 1].is_ascii_alphanumeric()) {
            continue;
        }
        let digits: String = text[at + 1..]
            .chars()
            .take_while(char::is_ascii_digit)
            .collect();
        if let Ok(line) = digits.parse() {
            cited.insert(line);
        }
    }
    cited
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
/// earlier `or fail` site — a lookup miss or a `require` guard — shadows it.
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
            let earlier = earlier_failure_site(&body[..at]);
            assert!(
                earlier.is_none(),
                "corpus/programs/{0}/: example on line {line} declares `fails {failure}`, but rule `{rule}` reaches an earlier `or fail` on the empty store before the `{site}` — the declared failure can never be the one a caller sees: {earlier:?}",
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

/// The first `or fail` site ahead of `before` — the part of a rule body before a
/// declared `or fail <Name>` — that shadows it: a `let x = <table>[<key>] or fail
/// …;` miss or a `require … or fail …;` guard. On the empty store every table
/// read misses, so any earlier `or fail` aborts before the declared failure is
/// reached and that failure can never be the one a caller sees. Comments are
/// stripped: a comment mentioning `or fail` is not a site.
fn earlier_failure_site(before: &str) -> Option<String> {
    before
        .lines()
        .map(strip_comment)
        .find(|line| line.contains("or fail "))
        .map(|line| line.trim().to_owned())
}

/// A lookup miss shadows a later failure just as a `require` guard does — the
/// defect class this check exists for (`corpus/programs/README.md`).
#[test]
fn an_earlier_lookup_miss_shadows_a_later_failure() {
    let lookup_first =
        "let t = tasks[id] or fail NotFound;\n  require t.owner == by or fail Forbidden;\n";
    assert_eq!(
        earlier_failure_site(lookup_first).as_deref(),
        Some("let t = tasks[id] or fail NotFound;"),
        "a `let … or fail` ahead of a `require … or fail` must count as an earlier failure site"
    );

    let guard_first = "require users[by].permission == \"admin\" or fail Forbidden;\n  let t = tasks[id] or fail NotFound;\n";
    assert_eq!(
        earlier_failure_site(guard_first).as_deref(),
        Some("require users[by].permission == \"admin\" or fail Forbidden;")
    );

    // A comment mentioning a failure is not a site, and an empty body has none.
    assert_eq!(
        earlier_failure_site("// or fail NotFound is not reached\n"),
        None
    );
    assert_eq!(earlier_failure_site(""), None);
}

/// `#7` is derived from a rule body, not from the `module …;` header: a module
/// whose only `;` is its header does not use the statement separator, and one
/// whose body carries two statements does. Without the first case the anchor can
/// never disagree with the source and the exactness claim is unfalsifiable.
#[test]
fn the_statement_separator_anchor_excludes_the_module_header() {
    let header_only = "module todo_basic @1;\n\nrule noop() {\n}\n";
    assert!(
        !derived_row(7, header_only),
        "a module whose only `;` is its `module …;` header must not claim #7"
    );

    let one_statement =
        "module todo_basic @1;\n\nrule noop() {\n  let t = tasks[id] or fail NotFound;\n}\n";
    assert!(
        !derived_row(7, one_statement),
        "one `;`-terminated statement is not the separator being exercised"
    );

    let two_statements = "module todo_basic @1;\n\nrule complete() {\n  let t = tasks[id] or fail NotFound;\n  require t.owner == by or fail Forbidden;\n}\n";
    assert!(
        derived_row(7, two_statements),
        "two `;`-terminated statements in one rule body must claim #7"
    );
}

/// A row that carries a resolution-link column again is malformed, not silently
/// accepted: the link lives in the index alone, so the twenty program READMEs
/// cannot drift into stale copies of it.
#[test]
fn an_anchor_row_carries_no_resolution_link_column() {
    let linked = "| Issue | Anchor (resolution link) | Used? |\n|---|---|---|\n| #7 statement separator | #7 | YES — `;` at :17 |\n";
    assert_eq!(
        anchor_row(linked, 7),
        None,
        "a three-column anchor row must be rejected, not read positionally"
    );

    let plain = "| Issue | Used? |\n|---|---|\n| #7 statement separator | YES — `;` at :17 |\n";
    assert_eq!(anchor_row(plain, 7), Some(true));
}

/// Every `:N` citation in an anchor row names a real line of the module, and a
/// `YES` row for a derived anchor cites exactly its evidence lines.
///
/// This is the invariant the rows kept losing: `YES`/`NONE` is derived, but the
/// citations inside the cell are prose, so editing a module rotates them and the
/// row then points at a blank line, a comment or the wrong construct. Checking
/// only `YES`/`NONE` left that drift to review — which is how these rows rotted
/// across rounds while the derived column stayed green. Extra citations are
/// rejected too, so a row cannot hedge with prose that happens to carry `:N`.
/// `#6` and `#8` have no evidence set to be exact against; their citations only
/// have to name real code.
#[test]
fn anchor_row_citations_point_at_their_evidence() -> Result<(), String> {
    let mut checked = 0;
    for program in programs()? {
        let module: Vec<&str> = program.source.lines().collect();
        for anchor in ALL_ANCHORS {
            let Some((readme_line, used)) = anchor_cell(&program.readme, anchor) else {
                continue;
            };
            let cited = cited_lines(&used);
            for line in &cited {
                assert!(
                    *line > 0 && *line <= module.len(),
                    "corpus/programs/{0}/README.md:{readme_line}: the `#{anchor}` row cites :{line}, but the module has only {1} lines",
                    program.name,
                    module.len()
                );
                assert!(
                    !strip_comment(module[line - 1]).trim().is_empty(),
                    "corpus/programs/{0}/README.md:{readme_line}: the `#{anchor}` row cites :{line}, which is blank or a comment, so it no longer names evidence",
                    program.name
                );
                checked += 1;
            }
            if !used.starts_with("YES") {
                continue;
            }
            let evidence = evidence_lines(anchor, &program.source);
            if evidence.is_empty() {
                // `#6` and `#8` have no derived evidence set, so their citations
                // are a maintainer's pointer, only required to name real code.
                continue;
            }
            let uncited: Vec<String> = evidence
                .difference(&cited)
                .map(|line| format!(":{line}"))
                .collect();
            assert!(
                uncited.is_empty(),
                "corpus/programs/{0}/README.md:{readme_line}: the `#{anchor}` row is YES but does not cite {uncited:?} — every evidence line must be cited",
                program.name
            );
            let extra: Vec<String> = cited
                .difference(&evidence)
                .map(|line| format!(":{line}"))
                .collect();
            assert!(
                extra.is_empty(),
                "corpus/programs/{0}/README.md:{readme_line}: the `#{anchor}` row cites {extra:?} as evidence, but no line there carries the `#{anchor}` construct — a citation names its evidence or it is prose",
                program.name
            );
        }
    }
    assert!(
        checked > 0,
        "the anchor rows must carry line citations for this check to mean anything"
    );
    Ok(())
}

/// A `:N` citation is told apart from the two colons that are not line numbers:
/// an ascribed id literal (`#12 : Id<Task>`) and a spec link
/// (`docs/spec/v0.1.md:640`). Reading either as a module line would make the
/// citation check pass on text that names no line at all.
#[test]
fn cited_lines_reads_module_lines_and_not_ascriptions() {
    assert_eq!(
        cited_lines("`return` at :20"),
        BTreeSet::from([20]),
        "a bare `:N` is a module line"
    );
    assert_eq!(
        cited_lines("ascribed id literal :9, :10, :11"),
        BTreeSet::from([9, 10, 11]),
        "a comma-separated citation list names every line"
    );
    assert!(
        cited_lines("fixture bob: Id<User> = #12 : Id<Task>").is_empty(),
        "`#12 : Id<Task>` is an ascription, not a citation of line 12"
    );
    assert!(
        cited_lines("docs/spec/v0.1.md:640").is_empty(),
        "a spec link is not a citation of line 640"
    );
    assert_eq!(
        cited_lines("`:17`, :18 and https://example.test/a#b"),
        BTreeSet::from([17, 18]),
        "citations come from all the prose around them"
    );
}

/// Evidence for `#7` is an `or fail` site inside a rule body — not the
/// `module …;` header, not a `return`, and not a comment that mentions a
/// failure. A row that cites a line outside that set is citing the wrong
/// construct, which is the drift the citation check exists to catch.
#[test]
fn evidence_lines_name_the_construct_and_not_a_neighbour() {
    let source = "module todo_basic @1;\n\n// `or fail NotFound` here is prose\nrule complete() {\n  let t = tasks[id] or fail NotFound;\n  return t;\n}\n\nexample complete() fails NotFound;\n";
    assert_eq!(
        evidence_lines(7, source),
        BTreeSet::from([5]),
        "only the `or fail` statement inside the rule body is #7 evidence"
    );
    assert_eq!(
        evidence_lines(5, source),
        BTreeSet::from([6]),
        "#5 evidence is the `return` statement"
    );
    assert!(
        evidence_lines(9, source).is_empty(),
        "a module with no `needs` clause has no #9 evidence to cite"
    );
}

/// A `YES` `#15` row cites the `example` lines, and each of those examples takes
/// only declared `fixture` arguments — the corpus rule in `corpus/README.md`
/// (fixture-only example arguments, issue #15). A literal or a computed
/// expression as an `example` argument is what the rule forbids, and it also
/// breaks the argument-token reading this list is derived from.
#[test]
fn example_arguments_are_declared_fixtures() -> Result<(), String> {
    let mut checked = 0;
    for program in programs()? {
        let fixtures: BTreeSet<&str> = program
            .source
            .lines()
            .filter_map(|line| line.trim().strip_prefix("fixture "))
            .filter_map(|rest| rest.split(':').next())
            .map(str::trim)
            .collect();
        let examples = evidence_lines(15, &program.source);
        for line in &examples {
            let text = program
                .source
                .lines()
                .nth(line - 1)
                .unwrap_or_default()
                .trim();
            let args = text
                .split_once('(')
                .and_then(|(_, rest)| rest.split_once(')'))
                .map(|(args, _)| args)
                .unwrap_or_default();
            for arg in args.split(',').map(str::trim).filter(|arg| !arg.is_empty()) {
                assert!(
                    fixtures.contains(arg),
                    "corpus/programs/{0}/: the example on line {line} passes `{arg}`, which is not a declared `fixture` — issue #15 requires fixture-only example arguments",
                    program.name
                );
                checked += 1;
            }
        }
    }
    assert!(
        checked > 0,
        "the corpus must declare at least one example argument to check"
    );
    Ok(())
}

/// Every anchor row any program carries is `| <issue> | <used?> |` — exactly two
/// cells — for all seven anchors, not only the derived ones.
///
/// `anchor_rows_match_the_program_source` only walks `DERIVED_ANCHORS`, so a
/// `#6`, `#8` or `#15` row that regrew a resolution-link column would keep that
/// test green while re-introducing the twenty stale copies the link-free row
/// exists to prevent. `anchor_cell` already rejects the extra cell; this makes
/// that rejection reach the non-derived rows.
#[test]
fn every_anchor_row_keeps_the_link_out_of_the_readme() -> Result<(), String> {
    let mut checked = 0;
    for program in programs()? {
        for anchor in ALL_ANCHORS {
            let prefix = format!("#{anchor} ");
            let found = program.readme.lines().any(|line| {
                let cells: Vec<&str> = line.split('|').collect();
                line.trim_start().starts_with('|')
                    && cells.len() > 1
                    && cells[1].trim().starts_with(&prefix)
            });
            assert!(
                found,
                "corpus/programs/{0}/README.md must carry a `#{anchor}` anchor row",
                program.name
            );
            assert!(
                anchor_row(&program.readme, anchor).is_some(),
                "corpus/programs/{0}/README.md: the `#{anchor}` row must be `| <issue> | YES…/NONE… |` — a resolution-link column belongs in the index alone",
                program.name
            );
            checked += 1;
        }
    }
    assert_eq!(
        checked,
        programs()?.len() * ALL_ANCHORS.len(),
        "every program must be checked for every anchor"
    );
    Ok(())
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
