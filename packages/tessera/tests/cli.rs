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
        /// Make the cargo-deny stub report an unreadable advisory database
        /// instead of a scanner failure.
        broken_advisory_db: bool,
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
                // bin/setup and bin/check read the cargo-deny version with
                // awk: without it the sandbox cannot tell a pinned install
                // from a missing one.
                ("awk", "/usr/bin/awk"),
            ] {
                symlink(target, root.join("tools").join(name)).unwrap();
            }
            // cargo-deny is stubbed too: the advisory step of bin/check starts
            // with `command -v cargo-deny`, and a missing binary is exit 75.
            // The stub is what makes the scan itself run; the branch where the
            // binary is ABSENT is covered by a sandbox that drops it.
            for tool in ["cargo", "cargo-deny", "rustup", "rustc", "git", "jq"] {
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
if [[ "${SCRIPT_DB_FAIL:-0}" == "1" && "$call" == "cargo deny --locked check advisories" ]]; then
    printf 'error: failed to load advisory database\n' >&2
    exit 1
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
            Self {
                root,
                broken_advisory_db: false,
            }
        }

        /// Make the cargo-deny stub report an unreadable advisory database:
        /// `cargo deny --locked check advisories` exits 1 with "failed to
        /// load advisory database" — the scanner ran, the database did not
        /// load.
        fn break_advisory_db(&mut self) {
            self.broken_advisory_db = true;
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
                .env("SCRIPT_FAIL", fail)
                .env(
                    "SCRIPT_DB_FAIL",
                    if self.broken_advisory_db { "1" } else { "0" },
                )
                // bin/setup looks for the pinned cargo-deny in $CARGO_HOME/bin.
                // Keep HOME and CARGO_HOME inside the sandbox, so a cargo-deny
                // installed on this machine cannot change the call list.
                .env("HOME", &self.root)
                .env("CARGO_HOME", self.root.join("cargo-home"));
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

        /// Take cargo-deny off the sandbox PATH: `command -v cargo-deny` has
        /// to fail, which is the "the scanner is missing" environment branch
        /// of bin/check.
        fn remove_cargo_deny_from_path(&self) {
            fs::remove_file(self.root.join("tools").join("cargo-deny")).unwrap();
        }

        /// Put a cargo-deny reporting `version` into the sandbox `CARGO_HOME` —
        /// the install root bin/setup looks in before it installs anything.
        fn cargo_deny_in_cargo_home(&self, version: &str) {
            let bin = self.root.join("cargo-home").join("bin");
            fs::create_dir_all(&bin).unwrap();
            let path = bin.join("cargo-deny");
            fs::write(
                &path,
                format!("#!/bin/bash\nprintf 'cargo-deny {version}\\n'\n"),
            )
            .unwrap();
            fs::set_permissions(path, fs::Permissions::from_mode(0o755)).unwrap();
        }
    }

    impl Drop for Sandbox {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.root);
        }
    }

    /// The pinned cargo-deny install `bin/setup` runs.
    const CARGO_DENY_INSTALL: &str = "cargo install --locked cargo-deny --version 0.20.2 --force";

    const SETUP: &[&str] = &[
        "rustup show active-toolchain",
        "rustc --version",
        "git config core.symlinks true",
        "cargo fetch --quiet --locked",
        "cargo build --quiet --workspace --locked",
        CARGO_DENY_INSTALL,
        "cargo deny --version",
    ];
    const CHECK: &[&str] = &[
        "cargo fmt --all -- --check",
        "cargo clippy --workspace --all-targets --locked -- -D warnings",
        "cargo test --workspace --locked --quiet",
        "cargo deny --version",
        "cargo deny fetch db",
        "cargo deny --locked check advisories",
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

    /// `SETUP` without the cargo-deny install line: the call list of a
    /// `bin/setup` that finds the pinned version already in `$CARGO_HOME`.
    fn setup_calls_without_install() -> Vec<&'static str> {
        SETUP
            .iter()
            .copied()
            .filter(|call| *call != CARGO_DENY_INSTALL)
            .collect()
    }

    /// Where the injected-failure message has to be visible.
    #[derive(Clone, Copy)]
    enum Seen {
        Stderr,
        Stdout,
    }

    /// How a script must react to an injected failure on `call`.
    ///
    /// `None` means the script swallows the failure on purpose, so injecting
    /// there must not be expected to stop it. Otherwise: the exit status the
    /// script must end with, and the stream the injected-failure message has
    /// to reach.
    ///
    /// The stub always exits 42, but the scripts classify a failure before
    /// they exit (exit codes in .claude/skills/verify-change/SKILL.md):
    ///
    /// * `rustc --version` only labels the activated toolchain.
    /// * `cargo deny --version` only probes the version — `bin/setup` warns
    ///   about a shadowing cargo-deny, `bin/check` warns about an unknown
    ///   one; neither stops.
    /// * `bin/setup` turns a failed cargo-deny install into `die` — exit 1.
    /// * `bin/check` turns an advisory database it cannot fetch into 75: the
    ///   environment is blamed, not the change.
    /// * `bin/check` turns a reported advisory into 1 — the change is red —
    ///   and prints the scanner's own output on stdout.
    ///
    /// The exit-42 stub is not the only failure mode: the advisory database
    /// has one of its own (`scripts_check_treats_an_unreadable_advisory_database_as_an_environment_failure`).
    fn injected(call: &str) -> Option<(i32, Seen)> {
        match call {
            "rustc --version" | "cargo deny --version" => None,
            CARGO_DENY_INSTALL => Some((1, Seen::Stderr)),
            "cargo deny fetch db" => Some((75, Seen::Stderr)),
            "cargo deny --locked check advisories" => Some((1, Seen::Stdout)),
            _ => Some((42, Seen::Stderr)),
        }
    }

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
                // A failure the script swallows on purpose — see `injected`.
                let Some((exit, seen)) = injected(call) else {
                    continue;
                };
                let out = sandbox.run(script, call);
                assert_eq!(out.status.code(), Some(exit), "{script}: {out:?}");
                sandbox.assert_calls(&calls[..=index]);
                // `out` is moved into the two buffers below, so the debug
                // rendering the assertion messages use is taken first.
                let out_debug = format!("{out:?}");
                let stdout = String::from_utf8(out.stdout).unwrap();
                let stderr = String::from_utf8(out.stderr).unwrap();
                let reported = match seen {
                    Seen::Stderr => stderr.contains(call),
                    Seen::Stdout => stdout.contains(call),
                };
                assert!(reported, "{script}: {call} not reported: {out_debug}");
                assert!(!stdout.contains("green"));
            }
        }
    }

    #[test]
    fn scripts_treat_a_failed_version_probe_as_a_warning_not_a_stop() {
        let sandbox = Sandbox::new();
        // A failing `cargo deny --version` probe only means "version unknown":
        // bin/setup warns about a shadowing cargo-deny, bin/check warns about
        // an unknown one, and both carry on.
        for (script, calls) in [("setup", SETUP), ("check", CHECK)] {
            let out = sandbox.run(script, "cargo deny --version");
            assert!(out.status.success(), "{script}: {out:?}");
            sandbox.assert_calls(calls);
            // The warning is the point of the test: bin/check says which
            // version is pinned and that PATH has another one, bin/setup says
            // which cargo-deny is on PATH. Deleting either warning has to
            // fail here.
            let out_debug = format!("{out:?}");
            let stdout = String::from_utf8(out.stdout).unwrap();
            let stderr = String::from_utf8(out.stderr).unwrap();
            if script == "setup" {
                assert!(
                    stdout.contains("cargo deny on PATH"),
                    "{script}: {out_debug}"
                );
            } else {
                assert!(
                    stderr.contains("is pinned — run bin/setup"),
                    "{script}: {out_debug}"
                );
            }
        }
    }

    #[test]
    fn scripts_check_treats_an_unreadable_advisory_database_as_an_environment_failure() {
        let mut sandbox = Sandbox::new();
        // The scanner ran, but the advisory database could not be loaded:
        // bin/check must exit 75 (the environment), not 1 (a red change
        // blamed on an advisory that was never reported).
        sandbox.break_advisory_db();
        let out = sandbox.run("check", "");
        assert_eq!(out.status.code(), Some(75), "{out:?}");
        // The scan ran and its output was printed; the guards never did.
        sandbox.assert_calls(&CHECK[..6]);
        let out_debug = format!("{out:?}");
        let stdout = String::from_utf8(out.stdout).unwrap();
        let stderr = String::from_utf8(out.stderr).unwrap();
        assert!(
            stdout.contains("failed to load advisory database"),
            "{out_debug}"
        );
        assert!(
            stderr.contains("the RustSec advisory database could not be read"),
            "{out_debug}"
        );
        assert!(!stdout.contains("green"));
    }

    #[test]
    fn scripts_check_reports_a_missing_cargo_deny_as_an_environment_failure() {
        let sandbox = Sandbox::new();
        // No cargo-deny on PATH: the gate cannot grade the change, so it must
        // exit 75 (environment) — not 1 (a red change) — and must not run the
        // advisory scan at all.
        sandbox.remove_cargo_deny_from_path();
        let out = sandbox.run("check", "");
        assert_eq!(out.status.code(), Some(75), "{out:?}");
        sandbox.assert_calls(&CHECK[..3]);
        assert!(!String::from_utf8(out.stdout).unwrap().contains("green"));
    }

    #[test]
    fn scripts_setup_skips_the_cargo_deny_install_when_the_pinned_version_is_there() {
        let sandbox = Sandbox::new();
        // The pinned version already sits in the install root: re-installing
        // it would be wasted work, and the install line must not appear.
        sandbox.cargo_deny_in_cargo_home("0.20.2");
        let out = sandbox.run("setup", "");
        assert!(out.status.success(), "{out:?}");
        sandbox.assert_calls(&setup_calls_without_install());
    }

    #[test]
    fn scripts_setup_installs_the_pinned_cargo_deny_over_a_different_version() {
        let sandbox = Sandbox::new();
        // A different version is not the pinned one: the gate needs 0.20.2,
        // so the install has to run.
        sandbox.cargo_deny_in_cargo_home("0.19.0");
        let out = sandbox.run("setup", "");
        assert!(out.status.success(), "{out:?}");
        sandbox.assert_calls(SETUP);
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
