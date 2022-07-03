//! Command handler.

use anyhow::*;
use clap::Parser;
use errmap::ErrorMapping;
use serde::{Deserialize, Serialize};
use std::io::{self, Read};

macro_rules! format_info {
    ($name:ident::$ty:ident) => {
        concat!(
            "A [move_core_types::",
            stringify!($name),
            "::",
            stringify!($ty),
            "]. (see <https://docs.rs/mv-core-types/latest/move_core_types/",
            stringify!($name),
            "/struct.",
            stringify!($ty),
            ".html>)",
        )
    };
}

fn render_json<'a, I, O>(bytes: &'a [u8]) -> Result<String>
where
    I: Deserialize<'a>,
    O: From<I> + ?Sized + Serialize,
{
    let result: I = bcs::from_bytes(bytes)?;
    let output: O = result.into();
    serde_json::to_string_pretty(&output).with_context(|| "could not render input")
}

/// The format of the data.
#[derive(clap::Subcommand, Copy, Clone, Debug, PartialEq, Eq)]
pub enum KnownFormat {
    #[clap(name = "errmap", about = format_info!(errmap::ErrorMapping))]
    ErrorMapping,
}

impl KnownFormat {
    fn render(self, input: &[u8]) -> Result<String> {
        match self {
            KnownFormat::ErrorMapping => {
                render_json::<move_core_types::errmap::ErrorMapping, ErrorMapping>(input)
            }
        }
    }
}

/// Reads a BCS-encoded file from stdin and prints it.
#[derive(Clone, Debug, clap::Parser)]
#[clap(
    about,
    version,
    author,
    arg_required_else_help = true,
    disable_help_subcommand = true,
    subcommand_help_heading = "FORMATS"
)]
pub struct Opts {
    /// Format of the input data.
    #[clap(subcommand)]
    pub format: KnownFormat,
}

impl Opts {
    pub fn run() -> Result<()> {
        let opts = Opts::parse();

        let mut buf = vec![];
        io::stdin().read_to_end(&mut buf)?;

        let out = opts.format.render(&buf)?;
        println!("{}", &out);

        Ok(())
    }
}
