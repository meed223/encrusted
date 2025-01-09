mod cli;
mod theme;

use std::process::ExitCode;
use clap::Parser;
use cli::Encrusted;

fn main() -> ExitCode {
    let cli = Encrusted::parse();

    ExitCode::SUCCESS
}
