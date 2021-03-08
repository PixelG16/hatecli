//! Command-Line Interface.
//!
//! The [`App`] structure is representative of the command-line application as
//! a whole and is owner of all arguments, options and subcommands passed via
//! the CLI.

use url::Url;
use structopt::StructOpt;
use std::fs::File;

#[derive(StructOpt)]
#[structopt(about =
"hatecli is a command-line version of HateBin.")]
pub struct App
{
    /// Text to be uploaded to Hatebin.
    #[structopt(short, 
                long, 
                conflicts_with = "file",
                conflicts_with = "url",
                required_unless_one(&["file", "url"]))]
    text: Option<String>,

    /// File to be uploaded to Hatebin.
    #[structopt(short,
                long, 
                conflicts_with = "text",
                conflicts_with = "url",
                required_unless_one(&["text", "url"]),
                parse(try_from_str = File::open))]
    file: Option<File>,

    /// Content located at URL to be uploaded to Hatebin.
    #[structopt(short, 
                long, 
                conflicts_with = "file",
                conflicts_with = "text",
                required_unless_one(&["file", "text"]))]
    url: Option<Url>,

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

    /// Content located at URL to be uploaded to Hatebin.
    pub fn url(&self) -> &Option<Url>
    {
        &self.url
    }

    /// Force an uploaded or not, regardless of issues.
    pub fn force(&self) -> bool
    {
        self.force
    }
}