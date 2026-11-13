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
if [[ "${0##*/}" == rustc ]]; then printf 'rustc %s\n' "${STUB_RUSTC_VERSION:-stub}"; fi
if [[ "${0##*/}" == rustup && "${1:-}" == which && -n "${STUB_RUSTUP_WHICH:-}" ]]; then
  printf '%s\n' "$STUB_RUSTUP_WHICH"
fi
"#,
                )
                .unwrap();
                fs::set_permissions(path, fs::Permissions::from_mode(0o755)).unwrap();
            }
            Self { root }
        }

        /// Pin `channel` in the sandbox root and give rustup a rustc to answer
        /// `which --toolchain <channel> rustc` with. That rustc reports
        /// `version` and lives outside the ambient PATH, so only a script that
        /// puts the pinned toolchain first reaches it.
        fn pin_toolchain(&self, channel: &str, version: &str) {
            fs::write(
                self.root.join("rust-toolchain.toml"),
                format!("[toolchain]\nchannel = \"{channel}\"\n"),
            )
            .unwrap();
            let pinned = self.root.join("pinned");
            fs::create_dir_all(&pinned).unwrap();
            let rustc = pinned.join("rustc");
            fs::write(&rustc, PINNED_RUSTC_STUB.replace("{version}", version)).unwrap();
            fs::set_permissions(&rustc, fs::Permissions::from_mode(0o755)).unwrap();
        }

        fn run(&self, script: &str, fail: &str) -> Output {
            self.run_with(script, fail, &[])
        }

        fn run_with(&self, script: &str, fail: &str, env: &[(&str, &str)]) -> Output {
            fs::write(self.root.join("calls"), "").unwrap();
            let mut command = Command::new(self.root.join("bin").join(script));
            command
                .current_dir(self.root.join("elsewhere"))
                .env("PATH", self.root.join("tools"))
                .env("SCRIPT_LOG", self.root.join("calls"))
                .env("SCRIPT_FAIL", fail);
            for (key, value) in env {
                command.env(key, value);
            }
            command.output().unwrap()
        }

        /// Run a script in a sandbox whose root pins a toolchain: rustup answers
        /// `which --toolchain <channel> rustc` with the pinned rustc.
        fn run_pinned(&self, script: &str) -> Output {
            let rustc = self.root.join("pinned").join("rustc");
            self.run_with(
                script,
                "",
                &[("STUB_RUSTUP_WHICH", rustc.to_str().unwrap())],
            )
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
    /// The toolchain the scripts must select, whatever answers on PATH.
    const PINNED_CHANNEL: &str = "1.98.0";
    /// A rustc outside the ambient PATH: only a script that puts the pinned
    /// toolchain first can reach it.
    const PINNED_RUSTC_STUB: &str = r#"#!/bin/bash
set -eu
call="${0##*/} $*"
printf '%s|%s\n' "$PWD" "$call" >> "$SCRIPT_LOG"
printf 'rustc {version}\n'
"#;

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
    fn scripts_select_the_pinned_toolchain_before_their_work() {
        let sandbox = Sandbox::new();
        sandbox.pin_toolchain(PINNED_CHANNEL, PINNED_CHANNEL);
        for (script, calls) in [
            ("check", CHECK),
            ("fmt", &["cargo fmt --all"][..]),
            ("dev", &["cargo run -q -p tessera -- help"][..]),
        ] {
            let out = sandbox.run_pinned(script);
            assert!(out.status.success(), "{script}: {out:?}");
            let mut expected = vec![
                format!("rustup which --toolchain {PINNED_CHANNEL} rustc"),
                "rustc --version".to_string(),
            ];
            expected.extend(calls.iter().map(ToString::to_string));
            let expected: Vec<&str> = expected.iter().map(String::as_str).collect();
            sandbox.assert_calls(&expected);
        }
    }

    #[test]
    fn scripts_refuse_a_rustc_that_answers_another_version() {
        let sandbox = Sandbox::new();
        sandbox.pin_toolchain(PINNED_CHANNEL, "1.96.0");
        let which = format!("rustup which --toolchain {PINNED_CHANNEL} rustc");
        for script in ["check", "fmt", "dev"] {
            let out = sandbox.run_pinned(script);
            assert_eq!(out.status.code(), Some(75), "{script}: {out:?}");
            let stderr = String::from_utf8(out.stderr).unwrap();
            assert!(stderr.contains(PINNED_CHANNEL), "{script}: {stderr}");
            assert!(stderr.contains("1.96.0"), "{script}: {stderr}");
            assert!(
                !String::from_utf8(out.stdout).unwrap().contains("green"),
                "{script}"
            );
            // Exactly the two selection calls: the script refused before it
            // called cargo.
            sandbox.assert_calls(&[which.as_str(), "rustc --version"]);
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
