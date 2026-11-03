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

fn read_case(dir: &Path, name: &str) -> Result<Value, String> {
    let path = dir.join(name);
    let text = fs::read_to_string(&path)
        .map_err(|e| format!("cannot read case {}: {e}", path.display()))?;
    serde_json::from_str(&text)
        .map_err(|e| format!("case {} is not valid JSON: {e}", path.display()))
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

/// A case is accepted only when it validates against the schema *and* satisfies
/// the ordering rules.
fn check(case: &Value) -> Result<(), String> {
    let validator = schema()?;
    let mut msg = String::new();
    for e in validator.iter_errors(case) {
        let _ = writeln!(msg, "{}: {e}", e.instance_path());
    }
    if !msg.is_empty() {
        return Err(msg);
    }
    check_order(case)
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
        let first = read_case(&dir, name);
        let second = read_case(&dir, name);
        assert!(
            first.is_ok() && second.is_ok(),
            "case {name} must be readable"
        );
        let first = first.unwrap_or_default();
        let second = second.unwrap_or_default();
        assert_eq!(first, second, "case {name} must read back identically");
        assert_eq!(
            validator.is_valid(&first),
            validator.is_valid(&second),
            "case {name} must validate identically twice"
        );
    }
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
