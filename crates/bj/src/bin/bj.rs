//! A CLI for converting BCS-encoded messages into JSON.

use anyhow::*;
use bj::command::Opts;

fn main() -> Result<()> {
    Opts::run()
}
