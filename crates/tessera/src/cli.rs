//! Argument parsing and the command table for `tessera` (spec §11).
//! Every command the spec names is listed here with the milestone that ships it, so `help`
//! tells the truth about this build instead of hiding what is planned.

/// One `tessera` subcommand from the spec, with the milestone that delivers it.
pub struct Planned {
    pub name: &'static str,
    pub milestone: &'static str,
    pub summary: &'static str,
}

/// Spec §11 commands plus the package commands from `docs/spec/packages.md`.
pub const COMMANDS: &[Planned] = &[
    Planned {
        name: "dev",
        milestone: "M1",
        summary: "interpreter, hot reload, fast proofs, REPL",
    },
    Planned {
        name: "check",
        milestone: "M1",
        summary: "type check, proofs, examples, parity",
    },
    Planned {
        name: "fmt",
        milestone: "M1",
        summary: "canonical formatter (idempotent)",
    },
    Planned {
        name: "build",
        milestone: "M4",
        summary: "AOT build",
    },
    Planned {
        name: "fill",
        milestone: "M2",
        summary: "fill a typed hole with a checked candidate",
    },
    Planned {
        name: "add",
        milestone: "M6",
        summary: "add a git dependency",
    },
    Planned {
        name: "update",
        milestone: "M6",
        summary: "re-resolve dependencies (MVS)",
    },
    Planned {
        name: "audit",
        milestone: "M2",
        summary: "list the trust boundary",
    },
    Planned {
        name: "vendor",
        milestone: "M6",
        summary: "copy dependencies into the repo",
    },
    Planned {
        name: "release",
        milestone: "M6",
        summary: "check, compat diff, signed tag",
    },
    Planned {
        name: "why-dep",
        milestone: "M6",
        summary: "explain why a dependency is present",
    },
];

/// What the user asked for, after parsing.
#[derive(Debug, PartialEq, Eq)]
pub enum Invocation {
    Help,
    Version,
    Planned(&'static str),
    Unknown(String),
}

/// Parsed arguments: the invocation plus the global `--json` flag.
#[derive(Debug, PartialEq, Eq)]
pub struct Args {
    pub invocation: Invocation,
    pub json: bool,
}

/// Parses everything after the program name. Never fails: unknown input is an `Invocation`.
pub fn parse<I: IntoIterator<Item = String>>(args: I) -> Args {
    let mut json = false;
    let mut first: Option<String> = None;
    for arg in args {
        if arg == "--json" {
            json = true;
        } else if first.is_none() {
            first = Some(arg);
        }
    }
    let invocation = match first.as_deref() {
        None | Some("help" | "--help" | "-h") => Invocation::Help,
        Some("--version" | "-V" | "version") => Invocation::Version,
        Some(name) => COMMANDS.iter().find(|c| c.name == name).map_or_else(
            || Invocation::Unknown(name.to_owned()),
            |c| Invocation::Planned(c.name),
        ),
    };
    Args { invocation, json }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(args: &[&str]) -> Args {
        parse(args.iter().map(|s| (*s).to_owned()))
    }

    #[test]
    fn empty_is_help() {
        assert_eq!(p(&[]).invocation, Invocation::Help);
    }

    #[test]
    fn json_flag_anywhere() {
        let a = p(&["--json", "check"]);
        assert!(a.json);
        assert_eq!(a.invocation, Invocation::Planned("check"));
    }

    #[test]
    fn unknown_keeps_the_name() {
        assert_eq!(
            p(&["chek"]).invocation,
            Invocation::Unknown("chek".to_owned())
        );
    }

    #[test]
    fn command_names_are_unique() {
        for (i, a) in COMMANDS.iter().enumerate() {
            assert!(
                COMMANDS[i + 1..].iter().all(|b| b.name != a.name),
                "duplicate {}",
                a.name
            );
        }
    }
}
