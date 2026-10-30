//! End-to-end: the built binary's exit codes and `--json` output.

#![allow(
    clippy::unwrap_used,
    reason = "test helper: a failed spawn should fail the test"
)]

use std::process::Command;

#[cfg(unix)]
mod scripts {
    use std::fmt::Write;
    use std::fs;
    use std::os::unix::fs::{PermissionsExt, symlink};
    use std::path::PathBuf;
    use std::process::{Command, Output};
    use std::sync::atomic::{AtomicUsize, Ordering};

    static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

    struct Sandbox {
        root: PathBuf,
    }

    impl Sandbox {
        fn new() -> Self {
            let root = std::env::temp_dir().join(format!(
                "tessera-scripts-{}-{}",
                std::process::id(),
                NEXT_ID.fetch_add(1, Ordering::Relaxed)
            ));
            fs::create_dir(&root).unwrap();
            let root = root.canonicalize().unwrap();
            fs::create_dir(root.join("bin")).unwrap();
            fs::create_dir(root.join("tools")).unwrap();
            fs::create_dir(root.join("elsewhere")).unwrap();
            let source = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
            for script in ["setup", "dev", "check", "fmt", "stop"] {
                fs::copy(
                    source.join("bin").join(script),
                    root.join("bin").join(script),
                )
                .unwrap();
            }
            // Keep PATH isolated: no real cargo, rustup or git can be reached.
            for (name, target) in [
                ("bash", "/bin/bash"),
                ("dirname", "/usr/bin/dirname"),
                ("mkdir", "/bin/mkdir"),
            ] {
                symlink(target, root.join("tools").join(name)).unwrap();
            }
            for tool in ["cargo", "rustup", "rustc", "git", "jq"] {
                let path = root.join("tools").join(tool);
                fs::write(
                    &path,
                    r#"#!/bin/bash
set -eu
call="${0##*/} $*"
printf '%s|%s\n' "$PWD" "$call" >> "$SCRIPT_LOG"
if [[ "$call" == "${SCRIPT_FAIL:-}" ]]; then
    printf 'injected failure: %s\n' "$call" >&2
    exit 42
fi
if [[ "${0##*/}" == rustc ]]; then printf 'rustc stub\n'; fi
"#,
                )
                .unwrap();
                fs::set_permissions(path, fs::Permissions::from_mode(0o755)).unwrap();
            }
            Self { root }
        }

        fn run(&self, script: &str, fail: &str) -> Output {
            fs::write(self.root.join("calls"), "").unwrap();
            Command::new(self.root.join("bin").join(script))
                .current_dir(self.root.join("elsewhere"))
                .env("PATH", self.root.join("tools"))
                .env("SCRIPT_LOG", self.root.join("calls"))
                .env("SCRIPT_FAIL", fail)
                .output()
                .unwrap()
        }

        fn assert_calls(&self, expected: &[&str]) {
            let actual = fs::read_to_string(self.root.join("calls")).unwrap();
            let mut expected_calls = String::new();
            for call in expected {
                writeln!(expected_calls, "{}|{call}", self.root.display()).unwrap();
            }
            assert_eq!(actual, expected_calls);
        }
    }

    impl Drop for Sandbox {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.root);
        }
    }

    const SETUP: &[&str] = &[
        "rustup show active-toolchain",
        "rustc --version",
        "git config core.symlinks true",
        "cargo fetch --quiet --locked",
        "cargo build --quiet --workspace --locked",
    ];
    const CHECK: &[&str] = &[
        "cargo fmt --all -- --check",
        "cargo clippy --workspace --all-targets --locked -- -D warnings",
        "cargo test --workspace --locked --quiet",
        "cargo xtask all",
    ];

    #[test]
    fn scripts_setup_is_locked_root_relative_and_repeatable() {
        let sandbox = Sandbox::new();
        for _ in 0..2 {
            let out = sandbox.run("setup", "");
            assert!(out.status.success(), "{out:?}");
            sandbox.assert_calls(SETUP);
            assert!(sandbox.root.join("tmp").is_dir());
        }
    }

    #[test]
    fn scripts_check_preserves_gate_order() {
        let sandbox = Sandbox::new();
        let out = sandbox.run("check", "");
        assert!(out.status.success(), "{out:?}");
        sandbox.assert_calls(CHECK);
        assert!(String::from_utf8(out.stdout).unwrap().contains("green"));
    }

    #[test]
    fn scripts_propagate_failures_without_running_later_phases() {
        let sandbox = Sandbox::new();
        for (script, calls) in [
            ("setup", SETUP),
            ("check", CHECK),
            ("dev", &["cargo run -q -p tessera -- help"][..]),
            ("fmt", &["cargo fmt --all"][..]),
        ] {
            for (index, call) in calls.iter().enumerate() {
                // rustc is only used to label the activated toolchain, not a gate phase.
                if *call == "rustc --version" {
                    continue;
                }
                let out = sandbox.run(script, call);
                assert_eq!(out.status.code(), Some(42), "{script}: {out:?}");
                sandbox.assert_calls(&calls[..=index]);
                assert!(String::from_utf8(out.stderr).unwrap().contains(call));
                assert!(!String::from_utf8(out.stdout).unwrap().contains("green"));
            }
        }
    }

    #[test]
    fn scripts_dev_and_fmt_invoke_the_workspace_commands() {
        let sandbox = Sandbox::new();
        for (script, call) in [
            ("dev", "cargo run -q -p tessera -- help"),
            ("fmt", "cargo fmt --all"),
        ] {
            let out = sandbox.run(script, "");
            assert!(out.status.success(), "{out:?}");
            sandbox.assert_calls(&[call]);
        }
    }

    #[test]
    fn scripts_stop_is_an_idempotent_no_op() {
        let sandbox = Sandbox::new();
        let data = sandbox.root.join("keep-data");
        fs::write(&data, "preserved").unwrap();
        for _ in 0..2 {
            let out = sandbox.run("stop", "");
            assert!(out.status.success(), "{out:?}");
            sandbox.assert_calls(&[]);
            assert_eq!(fs::read_to_string(&data).unwrap(), "preserved");
        }
    }
}

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
fn scripts_cli_help_exits_zero_with_usage_and_milestones() {
    for args in [&["help"][..], &["--help"][..], &[][..]] {
        let (code, out) = tessera(args);
        assert_eq!(code, 0, "{out}");
        assert!(out.contains("usage: tessera <command> [--json]"), "{out}");
        assert!(out.contains("commands:"), "{out}");
        assert!(out.contains("M1"), "{out}");
        assert!(out.contains("M6"), "{out}");
    }
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
