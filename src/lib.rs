//! coldframe's core: pure functions from data to data. The edge (`src/main.rs`, and later the
//! files under `src/edge/`) reads the disk, the clock, the process table and `gh`, and passes
//! what it read in here (docs/architecture.md, "Shape").

/// The usage line every usage error prints. Commands join it as they land.
pub const USAGE: &str = "usage: coldframe --version | --help";

/// What one invocation asks for, read from its arguments, the program name left out.
#[derive(Debug, PartialEq, Eq)]
pub enum Invocation {
    Version,
    Help,
    /// A usage error, with what was wrong.
    Usage(String),
}

pub fn version() -> String {
    format!("coldframe {}", env!("CARGO_PKG_VERSION"))
}

pub fn parse(args: &[String]) -> Invocation {
    match args {
        [only] if only == "--version" => Invocation::Version,
        [only] if only == "--help" => Invocation::Help,
        [] => Invocation::Usage("no command given".to_string()),
        [flag, ..] if flag == "--version" || flag == "--help" => {
            Invocation::Usage(format!("{flag} takes no arguments"))
        }
        [first, ..] => Invocation::Usage(format!("not a command: {first}")),
    }
}
