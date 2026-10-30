//! End-to-end: the built binary's exit codes and `--json` output.

#![allow(
    clippy::unwrap_used,
    reason = "test helper: a failed spawn should fail the test"
)]

use std::process::Command;

fn tessera(args: &[&str]) -> (i32, String) {
    let out = Command::new(env!("CARGO_BIN_EXE_tessera"))
        .args(args)
        .output()
        .unwrap();
    (
        out.status.code().unwrap(),
        String::from_utf8(out.stdout).unwrap(),
    )
}

#[test]
fn version_exits_zero() {
    let (code, out) = tessera(&["--version"]);
    assert_eq!(code, 0);
    assert!(out.starts_with("tessera "));
}

#[test]
fn planned_command_names_its_milestone_as_json() {
    let (code, out) = tessera(&["check", "--json"]);
    assert_eq!(code, 2);
    assert!(out.contains(r#""code":"E-CLI-002""#), "{out}");
    assert!(out.contains("M1"), "{out}");
}

#[test]
fn unknown_command_is_e_cli_001() {
    let (code, out) = tessera(&["frobnicate"]);
    assert_eq!(code, 2);
    assert!(out.starts_with("error[E-CLI-001]"), "{out}");
}
