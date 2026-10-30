//! `tessera` entry point: parse, dispatch, map the outcome to an exit code.
//! Commands not yet built answer with a stable diagnostic code instead of silence (spec §11).

mod cli;

use std::fmt::Write as _;
use std::io::{self, Write};
use std::process::ExitCode;

use cli::{COMMANDS, Invocation};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> ExitCode {
    let args = cli::parse(std::env::args().skip(1));
    let mut out = io::stdout().lock();
    let result = match &args.invocation {
        Invocation::Help => help(&mut out).map(|()| ExitCode::SUCCESS),
        Invocation::Version => writeln!(out, "tessera {VERSION}").map(|()| ExitCode::SUCCESS),
        Invocation::Planned(name) => {
            let milestone = COMMANDS
                .iter()
                .find(|c| c.name == *name)
                .map_or("?", |c| c.milestone);
            let msg = format!("`tessera {name}` is planned for {milestone} and not in this build");
            diagnostic(&mut out, args.json, "E-CLI-002", &msg).map(|()| ExitCode::from(2))
        }
        Invocation::Unknown(name) => {
            let msg = format!("unknown command `{name}`; run `tessera help`");
            diagnostic(&mut out, args.json, "E-CLI-001", &msg).map(|()| ExitCode::from(2))
        }
    };
    result.unwrap_or(ExitCode::FAILURE)
}

fn help(out: &mut impl Write) -> io::Result<()> {
    writeln!(
        out,
        "tessera {VERSION} — declare types, rules and laws; the compiler proves them\n"
    )?;
    writeln!(out, "usage: tessera <command> [--json]\n\ncommands:")?;
    for c in COMMANDS {
        writeln!(out, "  {:<9} {:<4} {}", c.name, c.milestone, c.summary)?;
    }
    writeln!(
        out,
        "\nspec: https://github.com/developerz-ai/tessera/blob/main/docs/spec/v0.1.md"
    )
}

fn diagnostic(out: &mut impl Write, json: bool, code: &str, message: &str) -> io::Result<()> {
    if json {
        writeln!(
            out,
            r#"{{"code":"{code}","severity":"error","message":"{}"}}"#,
            escape_json(message)
        )
    } else {
        writeln!(out, "error[{code}]: {message}")
    }
}

fn escape_json(s: &str) -> String {
    let mut o = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '"' => o.push_str("\\\""),
            '\\' => o.push_str("\\\\"),
            c if u32::from(c) < 0x20 => {
                let _ = write!(o, "\\u{:04x}", u32::from(c));
            }
            c => o.push(c),
        }
    }
    o
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_diagnostic_escapes_user_input() {
        let mut buf = Vec::new();
        diagnostic(&mut buf, true, "E-CLI-001", "bad `a\"b\\c\n`").unwrap();
        let s = String::from_utf8(buf).unwrap();
        assert_eq!(
            s,
            "{\"code\":\"E-CLI-001\",\"severity\":\"error\",\"message\":\"bad `a\\\"b\\\\c\\u000a`\"}\n"
        );
    }

    #[test]
    fn human_diagnostic_carries_code() {
        let mut buf = Vec::new();
        diagnostic(&mut buf, false, "E-CLI-002", "x").unwrap();
        assert_eq!(String::from_utf8(buf).unwrap(), "error[E-CLI-002]: x\n");
    }
}
