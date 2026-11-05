//! The edge: read the arguments, ask the core, print, and exit 0 when done or 2 on a usage
//! error (docs/architecture.md, "Exit codes").

use std::process::ExitCode;

use coldframe::{Invocation, USAGE, parse, version};

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match parse(&args) {
        Invocation::Version => {
            println!("{}", version());
            ExitCode::SUCCESS
        }
        Invocation::Help => {
            println!("{USAGE}");
            ExitCode::SUCCESS
        }
        Invocation::Usage(why) => {
            eprintln!("coldframe: {why}\n{USAGE}");
            ExitCode::from(2)
        }
    }
}
