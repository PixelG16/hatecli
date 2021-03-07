//! Command-Line Interface.
//!
//! The [`App`] structure is representative of the command-line application as
//! a whole and is owner of all arguments, options and subcommands passed via
//! the CLI.

use structopt::StructOpt;
use std::fs::File;

#[derive(StructOpt)]
#[structopt(about =
"hatecli is a command-line version of HateBin.")]
pub struct App
{
    /// Text to be uploaded to Hatebin.
    #[structopt(short, long, required_unless = "file")]
    text: Option<String>,

    /// File to be uploaded to Hatebin.
    #[structopt(short, long, conflicts_with = "text", parse(try_from_str = File::open))]
    file: Option<File>,

    /// Force an upload or not, regardless of issues.
    #[structopt(short = "F", long)]
    force: bool
}

impl App
{
    /// Text to be uploaded to Hatebin.
    pub fn text(&self) -> &Option<String>
    {
        &self.text
    }

    /// File to be uploaded to Hatebin.
    pub fn file(&self) -> &Option<File>
    {
        &self.file
    }

    /// Force an uploaded or not, regardless of issues.
    pub fn force(&self) -> bool
    {
        self.force
    }
}