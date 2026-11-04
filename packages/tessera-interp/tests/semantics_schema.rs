//! Conformance suite for the semantics test-suite format (spec §5, WP C).
//!
//! The semantics test suite is the interface of WP C (`tessera-interp`), and it
//! is **provisional**: the format is not frozen yet, so this suite, the schema
//! and the corpus cases may be reworked before the freeze point, and pre-freeze
//! rework is accepted. Its shape is given by
//! [`schema/semantics-case.schema.json`](../schema/semantics-case.schema.json)
//! and its normative meaning by
//! [`schema/semantics-case.contract.md`](../schema/semantics-case.contract.md).
//!
//! Every case under `corpus/semantics/` MUST validate against the schema, and
//! every case that violates the contract MUST be rejected. A test that cannot
//! fail is not a test (`.claude/agents/test-author.md`), so each negative case
//! here breaks exactly one rule and asserts the violation is caught.
//!
//! The workspace lints deny `unwrap`/`expect`/`panic!` even in test targets,
//! so the helpers below return `Result` and the tests report through
//! `assert!`.

use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

use jsonschema::Validator;
use serde_json::{Value, json};

/// The schema, compiled once per test binary.
fn schema() -> Result<Validator, String> {
    let path = schema_path();
    let text = fs::read_to_string(&path)
        .map_err(|e| format!("cannot read schema {}: {e}", path.display()))?;
    let value: Value = serde_json::from_str(&text)
        .map_err(|e| format!("schema {} is not valid JSON: {e}", path.display()))?;
    jsonschema::validator_for(&value)
        .map_err(|e| format!("semantics-case.schema.json is not a valid schema: {e}"))
}

/// `packages/tessera-interp/schema/semantics-case.schema.json`, found from the
/// crate manifest directory so the test does not depend on the current working
/// directory.
fn schema_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("schema/semantics-case.schema.json")
}

/// `corpus/semantics/`, found from the crate manifest directory.
fn corpus_dir() -> Result<PathBuf, String> {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .map(|root| root.join("corpus/semantics"))
        .ok_or_else(|| "crate is two levels below the repo root".to_owned())
}

/// The three worked cases. Exactly these three files exist under `corpus/semantics/`.
const CASES: [&str; 3] = [
    "successful-commit.json",
    "failed-call.json",
    "nondeterministic-inputs.json",
];

/// The raw bytes of a case file, read without parsing. The contract promises
/// that reading a case twice yields identical *bytes* (conformance item 3), so
/// the suite compares bytes before it parses: a comparison of parsed values
/// would hide a difference in whitespace or object-key order.
fn read_case_bytes(dir: &Path, name: &str) -> Result<Vec<u8>, String> {
    let path = dir.join(name);
    fs::read(&path).map_err(|e| format!("cannot read case {}: {e}", path.display()))
}

/// A case parsed from its bytes.
fn parse_case(bytes: &[u8], name: &str) -> Result<Value, String> {
    serde_json::from_slice(bytes).map_err(|e| format!("case {name} is not valid JSON: {e}"))
}

/// The contract's byte-identity guarantee (conformance item 3): two reads of a
/// case are identical when their bytes are. A comparison of parsed values would
/// hide a difference in whitespace or object-key order, so the suite compares
/// bytes.
fn identical_bytes(first: &[u8], second: &[u8]) -> bool {
    first == second
}

fn read_case(dir: &Path, name: &str) -> Result<Value, String> {
    let bytes = read_case_bytes(dir, name)?;
    parse_case(&bytes, name)
}

/// The contract rules that JSON Schema 2020-12 cannot express, because the
/// draft has no keyword for cross-item ordering. They are normative and are
/// enforced here, by the suite that ships with the schema.
fn check_order(case: &Value) -> Result<(), String> {
    let calls = case["calls"]
        .as_array()
        .ok_or_else(|| "calls is not an array".to_owned())?;
    for (position, call) in calls.iter().enumerate() {
        let index = call["index"]
            .as_u64()
            .ok_or_else(|| format!("call at position {position} has no integer index"))?;
        if index != position as u64 {
            return Err(format!(
                "call at position {position} carries index {index}; the index must equal the position"
            ));
        }
    }
    Ok(())
}

/// The calendar of a clock reading — the rule JSON Schema cannot express.
/// The reading fixes the type of the logged value: an `Instant` for `now`, a
/// calendar `Date` for `today`. The schema enforces that shape; only the suite
/// can reject a day that does not exist in its month (`2026-02-30`), because a
/// pattern cannot.
fn check_clock_dates(case: &Value) -> Result<(), String> {
    let Some(calls) = case["calls"].as_array() else {
        return Ok(());
    };
    for (position, call) in calls.iter().enumerate() {
        let Some(inputs) = call["inputs"].as_array() else {
            continue;
        };
        for (index, input) in inputs.iter().enumerate() {
            if input["kind"].as_str() != Some("clock") {
                continue;
            }
            let Some(value) = input["value"].as_str() else {
                // The schema rejects a clock value that is not a string.
                continue;
            };
            let Some(date) = value.get(..10) else {
                return Err(format!(
                    "call {position}, input {index}: a clock value carries no calendar date"
                ));
            };
            check_calendar_date(date).map_err(|e| {
                format!("call {position}, input {index}: a clock value must be a real date: {e}")
            })?;
        }
    }
    Ok(())
}

/// A calendar date `YYYY-MM-DD` must be a real date: the day must exist in its
/// month, and February has 29 days only in a leap year (proleptic Gregorian,
/// spec §2).
fn check_calendar_date(date: &str) -> Result<(), String> {
    let bytes = date.as_bytes();
    if bytes.len() != 10 || bytes[4] != b'-' || bytes[7] != b'-' {
        return Err(format!("`{date}` is not a YYYY-MM-DD date"));
    }
    let (Some(year), Some(month), Some(day)) = (
        parse_digits(&bytes[0..4]),
        parse_digits(&bytes[5..7]),
        parse_digits(&bytes[8..10]),
    ) else {
        return Err(format!("`{date}` is not a YYYY-MM-DD date"));
    };
    if !(1..=12).contains(&month) {
        return Err(format!("`{date}` has no month {month}"));
    }
    if !(1..=days_in_month(year, month)).contains(&day) {
        return Err(format!("`{date}` has no day {day} in month {month}"));
    }
    Ok(())
}

/// The days a month has in the proleptic Gregorian calendar.
fn days_in_month(year: u32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => 0,
    }
}

/// A leap year is divisible by 4, except the centuries, which are leap years
/// only when divisible by 400.
fn is_leap_year(year: u32) -> bool {
    year.is_multiple_of(4) && (!year.is_multiple_of(100) || year.is_multiple_of(400))
}

/// The number the ASCII digits spell, or `None` when they are not digits.
fn parse_digits(bytes: &[u8]) -> Option<u32> {
    let text = std::str::from_utf8(bytes).ok()?;
    if !text.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    text.parse::<u32>().ok()
}

/// A case is accepted only when it validates against the schema *and* satisfies
/// the rules the schema cannot express: the ordering rule and the calendar of
/// a clock reading.
fn check(case: &Value) -> Result<(), String> {
    let validator = schema()?;
    let mut msg = String::new();
    for e in validator.iter_errors(case) {
        let _ = writeln!(msg, "{}: {e}", e.instance_path());
    }
    if !msg.is_empty() {
        return Err(msg);
    }
    check_order(case)?;
    check_clock_dates(case)
}

/// Assert that a malformed case is rejected, and say which rule it broke.
#[track_caller]
fn assert_rejected(case: &Value, what: &str) {
    assert!(
        check(case).is_err(),
        "{what} must be rejected, but the case was accepted"
    );
}

/// A copy of a worked case with one node replaced.
///
/// A path segment is an object key or an array index, so `["calls", "0",
/// "result"]` reaches the result of the first call.
fn mutate(case: &Value, path: &[&str], replacement: Value) -> Result<Value, String> {
    let Some((head, rest)) = path.split_first() else {
        return Ok(replacement);
    };
    let mut out = case.clone();
    if let Value::Array(items) = &mut out {
        let idx: usize = head
            .parse()
            .map_err(|_| format!("path segment `{head}` is not an array index"))?;
        let child = items
            .get(idx)
            .cloned()
            .ok_or_else(|| format!("no element {idx} in the addressed array"))?;
        let new = mutate(&child, rest, replacement)?;
        if let Some(slot) = items.get_mut(idx) {
            *slot = new;
        }
        return Ok(out);
    }
    let obj = out
        .as_object_mut()
        .ok_or_else(|| "path segment addresses a non-object".to_owned())?;
    let child = obj
        .get(*head)
        .cloned()
        .ok_or_else(|| format!("no key `{head}` at the addressed object"))?;
    let new = mutate(&child, rest, replacement)?;
    obj.insert((*head).to_owned(), new);
    Ok(out)
}

// --- the worked cases ---------------------------------------------------

#[test]
fn every_worked_case_validates() {
    let dir = corpus_dir();
    assert!(dir.is_ok(), "corpus/semantics/ must exist");
    let dir = dir.unwrap_or_default();
    for name in CASES {
        let case = read_case(&dir, name);
        assert!(case.is_ok(), "worked case {name} must be readable");
        let case = case.unwrap_or_default();
        let verdict = check(&case);
        let reason = match verdict {
            Ok(()) => String::new(),
            Err(e) => e,
        };
        assert!(
            reason.is_empty(),
            "worked case {name} must satisfy the contract, but: {reason}"
        );
    }
}

#[test]
fn corpus_holds_exactly_the_three_worked_cases() {
    let dir = corpus_dir();
    assert!(dir.is_ok(), "corpus/semantics/ must exist");
    let dir = dir.unwrap_or_default();
    let entries = fs::read_dir(&dir);
    assert!(entries.is_ok(), "corpus/semantics/ must be readable");
    let mut found: Vec<String> = match entries {
        Ok(it) => it
            .filter_map(Result::ok)
            .map(|e| e.file_name().to_string_lossy().into_owned())
            .filter(|n| {
                Path::new(n)
                    .extension()
                    .is_some_and(|ext| ext.eq_ignore_ascii_case("json"))
            })
            .collect(),
        Err(_) => Vec::new(),
    };
    found.sort();
    let mut expected: Vec<String> = CASES.iter().map(|s| (*s).to_owned()).collect();
    expected.sort();
    assert_eq!(
        found, expected,
        "corpus/semantics/ holds exactly the three worked cases"
    );
}

/// The corpus is the ground truth for every backend, so the same case must
/// validate identically on a second read: no map iteration, no clock, no
/// randomness reaches the result (spec §1, P1).
#[test]
fn validation_is_deterministic() {
    let dir = corpus_dir();
    assert!(dir.is_ok(), "corpus/semantics/ must exist");
    let dir = dir.unwrap_or_default();
    let validator = schema();
    assert!(validator.is_ok(), "the schema must compile");
    let Ok(validator) = validator else {
        return;
    };
    for name in CASES {
        let first = read_case_bytes(&dir, name);
        let second = read_case_bytes(&dir, name);
        assert!(
            first.is_ok() && second.is_ok(),
            "case {name} must be readable"
        );
        let first = first.unwrap_or_default();
        let second = second.unwrap_or_default();
        // Bytes before values: the contract promises identical bytes, and a
        // comparison of parsed values would hide a difference in whitespace or
        // object-key order.
        assert!(
            identical_bytes(&first, &second),
            "case {name} must read back as identical bytes"
        );
        let first = parse_case(&first, name);
        let second = parse_case(&second, name);
        assert!(
            first.is_ok() && second.is_ok(),
            "case {name} must parse as JSON"
        );
        let first = first.unwrap_or_default();
        let second = second.unwrap_or_default();
        assert_eq!(first, second, "case {name} must parse to one value");
        assert_eq!(
            validator.is_valid(&first),
            validator.is_valid(&second),
            "case {name} must validate identically twice"
        );
    }
}

/// The determinism guarantee is stated in bytes: a case that differs only in
/// whitespace is the same value but not the same bytes, and the suite must see
/// the difference.
#[test]
fn a_whitespace_only_difference_is_not_byte_identity() {
    let dir = corpus_dir();
    assert!(dir.is_ok(), "corpus/semantics/ must exist");
    let dir = dir.unwrap_or_default();
    let bytes = read_case_bytes(&dir, "successful-commit.json");
    assert!(bytes.is_ok(), "successful-commit.json must be readable");
    let bytes = bytes.unwrap_or_default();
    let spaced = [bytes.as_slice(), b"\n"].concat();
    assert!(
        !identical_bytes(&bytes, &spaced),
        "a whitespace-only difference must not pass as byte identity"
    );
    let first = parse_case(&bytes, "successful-commit.json");
    let second = parse_case(&spaced, "successful-commit.json");
    assert!(
        first.is_ok() && second.is_ok(),
        "both variants must parse as JSON"
    );
    assert_eq!(
        first.unwrap_or_default(),
        second.unwrap_or_default(),
        "both variants parse to one value, which is why the guarantee is stated in bytes"
    );
}

// --- the three cases collectively cover the contract --------------------

#[test]
fn the_cases_cover_ok_failed_and_every_input_kind() {
    let dir = corpus_dir();
    assert!(dir.is_ok(), "corpus/semantics/ must exist");
    let dir = dir.unwrap_or_default();

    let ok_case = read_case(&dir, "successful-commit.json");
    assert!(ok_case.is_ok(), "successful-commit.json must be readable");
    let ok_case = ok_case.unwrap_or_default();
    assert!(
        ok_case["calls"][0]["result"]["ok"].is_object()
            || ok_case["calls"][0]["result"]["ok"].is_boolean(),
        "successful-commit must publish an Ok(v) result"
    );

    let failed_case = read_case(&dir, "failed-call.json");
    assert!(failed_case.is_ok(), "failed-call.json must be readable");
    let failed_case = failed_case.unwrap_or_default();
    assert_eq!(
        failed_case["calls"][0]["result"]["failed"], "NotFound",
        "failed-call must return Failed(Name)"
    );
    assert_eq!(
        failed_case["expected_state"], failed_case["initial_state"],
        "an aborted call leaves the persistent state unchanged (spec §5.1)"
    );
    let outputs = failed_case["expected_outputs"].as_array();
    assert!(
        outputs.is_some_and(Vec::is_empty),
        "an aborted call discards every buffered output (spec §5.1)"
    );
    let inputs = failed_case["calls"][0]["inputs"].as_array();
    assert!(
        inputs.is_some_and(|i| !i.is_empty()),
        "inputs consumed by an aborted call stay in the log (spec §5.1)"
    );

    let nondet = read_case(&dir, "nondeterministic-inputs.json");
    assert!(
        nondet.is_ok(),
        "nondeterministic-inputs.json must be readable"
    );
    let nondet = nondet.unwrap_or_default();
    let kinds: Vec<String> = nondet["calls"]
        .as_array()
        .map(|calls| {
            calls
                .iter()
                .flat_map(|c| c["inputs"].as_array().cloned().unwrap_or_default())
                .filter_map(|i| i["kind"].as_str().map(str::to_owned))
                .collect()
        })
        .unwrap_or_default();
    for kind in ["clock", "random", "id"] {
        assert!(
            kinds.iter().any(|k| k == kind),
            "the cases must log a {kind} input"
        );
    }
}

// --- malformed cases ----------------------------------------------------

#[test]
fn a_missing_required_field_is_rejected() {
    let dir = corpus_dir();
    assert!(dir.is_ok(), "corpus/semantics/ must exist");
    let dir = dir.unwrap_or_default();
    let case = read_case(&dir, "successful-commit.json");
    assert!(case.is_ok(), "successful-commit.json must be readable");
    let case = case.unwrap_or_default();

    for field in [
        "module",
        "initial_state",
        "calls",
        "expected_state",
        "expected_outputs",
    ] {
        let mut broken = case.clone();
        if let Some(obj) = broken.as_object_mut() {
            obj.remove(field);
        }
        assert_rejected(&broken, &format!("a case without `{field}`"));
    }

    // A call without a result is rejected too.
    let broken = mutate(
        &case,
        &["calls", "0"],
        json!({ "index": 0, "rule": "complete" }),
    );
    assert!(broken.is_ok(), "the mutation must apply");
    assert_rejected(&broken.unwrap_or_default(), "a call without a result");

    // A call without its index is rejected: the index carries the call's order.
    let broken = mutate(
        &case,
        &["calls", "0"],
        json!({ "rule": "complete", "result": { "ok": true } }),
    );
    assert!(broken.is_ok(), "the mutation must apply");
    assert_rejected(&broken.unwrap_or_default(), "a call without an index");

    // A call without its rule is rejected.
    let broken = mutate(
        &case,
        &["calls", "0"],
        json!({ "index": 0, "result": { "ok": true } }),
    );
    assert!(broken.is_ok(), "the mutation must apply");
    assert_rejected(&broken.unwrap_or_default(), "a call without a rule");
}

#[test]
fn a_mistagged_result_is_rejected() {
    let dir = corpus_dir();
    assert!(dir.is_ok(), "corpus/semantics/ must exist");
    let dir = dir.unwrap_or_default();
    let case = read_case(&dir, "successful-commit.json");
    assert!(case.is_ok(), "successful-commit.json must be readable");
    let case = case.unwrap_or_default();

    // Both tags at once.
    let broken = mutate(
        &case,
        &["calls", "0", "result"],
        json!({ "ok": true, "failed": "NotFound" }),
    );
    assert!(broken.is_ok(), "the mutation must apply");
    assert_rejected(&broken.unwrap_or_default(), "a result with both tags");

    // Neither tag.
    let broken = mutate(&case, &["calls", "0", "result"], json!({}));
    assert!(broken.is_ok(), "the mutation must apply");
    assert_rejected(&broken.unwrap_or_default(), "a result with no tag");

    // A wrong tag name.
    let broken = mutate(&case, &["calls", "0", "result"], json!({ "success": true }));
    assert!(broken.is_ok(), "the mutation must apply");
    assert_rejected(&broken.unwrap_or_default(), "an untagged result");

    // A failure name that is not an identifier.
    let broken = mutate(
        &case,
        &["calls", "0", "result"],
        json!({ "failed": "not a name" }),
    );
    assert!(broken.is_ok(), "the mutation must apply");
    assert_rejected(&broken.unwrap_or_default(), "a malformed failure name");
}

#[test]
fn a_misordered_call_sequence_is_rejected() {
    let dir = corpus_dir();
    assert!(dir.is_ok(), "corpus/semantics/ must exist");
    let dir = dir.unwrap_or_default();
    let case = read_case(&dir, "nondeterministic-inputs.json");
    assert!(
        case.is_ok(),
        "nondeterministic-inputs.json must be readable"
    );
    let case = case.unwrap_or_default();

    // The index no longer matches the position in the array.
    let broken = mutate(&case, &["calls", "1", "index"], json!(5));
    assert!(broken.is_ok(), "the mutation must apply");
    assert_rejected(
        &broken.unwrap_or_default(),
        "a call whose index does not match its position",
    );

    // The same call twice: the indices repeat.
    let broken = mutate(&case, &["calls", "1", "index"], json!(0));
    assert!(broken.is_ok(), "the mutation must apply");
    assert_rejected(&broken.unwrap_or_default(), "a repeated call index");

    // A negative index.
    let broken = mutate(&case, &["calls", "0", "index"], json!(-1));
    assert!(broken.is_ok(), "the mutation must apply");
    assert_rejected(&broken.unwrap_or_default(), "a negative call index");
}

#[test]
fn a_malformed_input_log_is_rejected() {
    let dir = corpus_dir();
    assert!(dir.is_ok(), "corpus/semantics/ must exist");
    let dir = dir.unwrap_or_default();
    let case = read_case(&dir, "nondeterministic-inputs.json");
    assert!(
        case.is_ok(),
        "nondeterministic-inputs.json must be readable"
    );
    let case = case.unwrap_or_default();

    // An input without a value.
    let broken = mutate(
        &case,
        &["calls", "0", "inputs", "0"],
        json!({ "kind": "id", "capability": "idgen" }),
    );
    assert!(broken.is_ok(), "the mutation must apply");
    assert_rejected(&broken.unwrap_or_default(), "an input without a value");

    // An input of an unknown kind.
    let broken = mutate(
        &case,
        &["calls", "0", "inputs", "0"],
        json!({ "kind": "weather", "value": 1 }),
    );
    assert!(broken.is_ok(), "the mutation must apply");
    assert_rejected(&broken.unwrap_or_default(), "an input of an unknown kind");

    // A clock reading without the reading it is.
    let broken = mutate(
        &case,
        &["calls", "0", "inputs", "1"],
        json!({ "kind": "clock", "capability": "clock", "value": "2026-10-22T09:30:00Z" }),
    );
    assert!(broken.is_ok(), "the mutation must apply");
    assert_rejected(
        &broken.unwrap_or_default(),
        "a clock input without a reading",
    );

    // A clock reading with a reading it does not have.
    let broken = mutate(
        &case,
        &["calls", "0", "inputs", "1"],
        json!({ "kind": "clock", "capability": "clock", "reading": "yesterday", "value": 1 }),
    );
    assert!(broken.is_ok(), "the mutation must apply");
    assert_rejected(&broken.unwrap_or_default(), "an unknown clock reading");

    // A clock reading without the capability it came from.
    let broken = mutate(
        &case,
        &["calls", "0", "inputs", "1"],
        json!({ "kind": "clock", "reading": "now", "value": "2026-10-22T09:30:00Z" }),
    );
    assert!(broken.is_ok(), "the mutation must apply");
    assert_rejected(
        &broken.unwrap_or_default(),
        "a clock input without a capability",
    );
}

#[test]
fn a_clock_reading_whose_value_is_not_its_reading_is_rejected() {
    let dir = corpus_dir();
    assert!(dir.is_ok(), "corpus/semantics/ must exist");
    let dir = dir.unwrap_or_default();
    let case = read_case(&dir, "nondeterministic-inputs.json");
    assert!(
        case.is_ok(),
        "nondeterministic-inputs.json must be readable"
    );
    let case = case.unwrap_or_default();

    // A `now` reading logged as something that is not an Instant.
    let broken = mutate(
        &case,
        &["calls", "0", "inputs", "1"],
        json!({ "kind": "clock", "capability": "clock", "reading": "now", "value": false }),
    );
    assert!(broken.is_ok(), "the mutation must apply");
    assert_rejected(
        &broken.unwrap_or_default(),
        "a `now` reading whose value is not an Instant",
    );

    // A `now` reading logged as a bare calendar date.
    let broken = mutate(
        &case,
        &["calls", "0", "inputs", "1"],
        json!({ "kind": "clock", "capability": "clock", "reading": "now", "value": "2026-10-22" }),
    );
    assert!(broken.is_ok(), "the mutation must apply");
    assert_rejected(
        &broken.unwrap_or_default(),
        "a `now` reading whose value is a Date",
    );

    // A `today` reading logged as an Instant.
    let broken = mutate(
        &case,
        &["calls", "1", "inputs", "0"],
        json!({ "kind": "clock", "capability": "clock", "reading": "today", "value": "2026-10-22T09:30:00Z" }),
    );
    assert!(broken.is_ok(), "the mutation must apply");
    assert_rejected(
        &broken.unwrap_or_default(),
        "a `today` reading whose value is an Instant",
    );

    // A day that does not exist in its month. The schema accepts the shape of
    // a date it cannot check, so the suite's calendar rule is what rejects
    // this one.
    let broken = mutate(
        &case,
        &["calls", "1", "inputs", "0"],
        json!({ "kind": "clock", "capability": "clock", "reading": "today", "value": "2026-02-30" }),
    );
    assert!(broken.is_ok(), "the mutation must apply");
    let broken = broken.unwrap_or_default();
    let verdict = check(&broken);
    let reason = verdict.err().unwrap_or_default();
    assert!(
        reason.contains("real date"),
        "the calendar rule must reject a day that does not exist, but: {reason}"
    );

    // An Instant whose day does not exist in its month.
    let broken = mutate(
        &case,
        &["calls", "0", "inputs", "1"],
        json!({ "kind": "clock", "capability": "clock", "reading": "now", "value": "2026-02-30T09:30:00Z" }),
    );
    assert!(broken.is_ok(), "the mutation must apply");
    assert_rejected(
        &broken.unwrap_or_default(),
        "a `now` reading whose day does not exist",
    );

    // A month that does not exist.
    let broken = mutate(
        &case,
        &["calls", "1", "inputs", "0"],
        json!({ "kind": "clock", "capability": "clock", "reading": "today", "value": "2026-13-01" }),
    );
    assert!(broken.is_ok(), "the mutation must apply");
    assert_rejected(
        &broken.unwrap_or_default(),
        "a `today` reading whose month does not exist",
    );

    // A leap day in a year that is not a leap year.
    let broken = mutate(
        &case,
        &["calls", "1", "inputs", "0"],
        json!({ "kind": "clock", "capability": "clock", "reading": "today", "value": "2026-02-29" }),
    );
    assert!(broken.is_ok(), "the mutation must apply");
    assert_rejected(
        &broken.unwrap_or_default(),
        "a `today` reading whose leap day does not exist",
    );
}

#[test]
fn a_malformed_tagged_id_is_rejected() {
    let dir = corpus_dir();
    assert!(dir.is_ok(), "corpus/semantics/ must exist");
    let dir = dir.unwrap_or_default();
    let case = read_case(&dir, "successful-commit.json");
    assert!(case.is_ok(), "successful-commit.json must be readable");
    let case = case.unwrap_or_default();

    // A well-formed tagged id is accepted.
    let good = mutate(
        &case,
        &["calls", "0", "args", "0"],
        json!({ "id": 7, "type": "Id<User>" }),
    );
    assert!(good.is_ok(), "the mutation must apply");
    let good = good.unwrap_or_default();
    assert!(
        check(&good).is_ok(),
        "a well-formed tagged id must be accepted"
    );

    // A record whose `type` field is not an id ascription is a record.
    let record = mutate(
        &case,
        &["calls", "0", "args", "0"],
        json!({ "id": 7, "type": "User" }),
    );
    assert!(record.is_ok(), "the mutation must apply");
    let record = record.unwrap_or_default();
    assert!(
        check(&record).is_ok(),
        "a record whose `type` field is not an id ascription must be accepted"
    );

    // A negative id number.
    let broken = mutate(
        &case,
        &["calls", "0", "args", "0"],
        json!({ "id": -1, "type": "Id<User>" }),
    );
    assert!(broken.is_ok(), "the mutation must apply");
    assert_rejected(
        &broken.unwrap_or_default(),
        "a tagged id with a negative number",
    );

    // A tagged id with a field that is not part of it.
    let broken = mutate(
        &case,
        &["calls", "0", "args", "0"],
        json!({ "id": 7, "type": "Id<User>", "extra": 1 }),
    );
    assert!(broken.is_ok(), "the mutation must apply");
    assert_rejected(
        &broken.unwrap_or_default(),
        "a tagged id with a field that is not part of it",
    );

    // A tagged id whose number is not a number.
    let broken = mutate(
        &case,
        &["calls", "0", "args", "0"],
        json!({ "id": "7", "type": "Id<User>" }),
    );
    assert!(broken.is_ok(), "the mutation must apply");
    assert_rejected(
        &broken.unwrap_or_default(),
        "a tagged id whose number is not a number",
    );
}

#[test]
fn a_malformed_state_or_output_is_rejected() {
    let dir = corpus_dir();
    assert!(dir.is_ok(), "corpus/semantics/ must exist");
    let dir = dir.unwrap_or_default();
    let case = read_case(&dir, "successful-commit.json");
    assert!(case.is_ok(), "successful-commit.json must be readable");
    let case = case.unwrap_or_default();

    // A state without its tables.
    let broken = mutate(&case, &["initial_state"], json!({}));
    assert!(broken.is_ok(), "the mutation must apply");
    assert_rejected(&broken.unwrap_or_default(), "a state without tables");

    // A table that is not a list of rows.
    let broken = mutate(
        &case,
        &["initial_state", "tables", "tasks"],
        json!({ "id": 1 }),
    );
    assert!(broken.is_ok(), "the mutation must apply");
    assert_rejected(&broken.unwrap_or_default(), "a table that is not a list");

    // An output without a stream.
    let broken = mutate(&case, &["expected_outputs", "0"], json!({ "value": 1 }));
    assert!(broken.is_ok(), "the mutation must apply");
    assert_rejected(&broken.unwrap_or_default(), "an output without a stream");

    // An unknown top-level key.
    let mut broken = case.clone();
    if let Some(obj) = broken.as_object_mut() {
        obj.insert("extra".to_owned(), json!(1));
    }
    assert_rejected(&broken, "an unknown top-level key");
}
