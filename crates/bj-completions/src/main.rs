//! A CLI for converting BCS-encoded messages into JSON.

use std::io;

use anyhow::*;
use bj::command::Opts;
use clap::CommandFactory;
use clap_complete::{generate, shells::Bash};

fn main() -> Result<()> {
    generate(Bash, &mut Opts::command(), "bj", &mut io::stdout());
    Ok(())
}
