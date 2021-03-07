//! Containing the [`main`] function / entry point to program.

use hatecli::cli::App;
use structopt::StructOpt;

/// Entry point to program.
///
/// After parsing values passed via the CLI, it calls the
/// [`run`](hatecli::run) function which will run the main logic of the 
/// program.
fn main() 
{
    // Get arguments, options and subcommands passed via CLI.
    let app = App::from_args();

    // Run main logic of program.
    hatecli::run(&app);
}