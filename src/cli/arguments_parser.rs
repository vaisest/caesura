use clap::Parser;

use crate::cli::command_arguments::CommandArguments;

/// Command line argument parser.
#[derive(Parser)]
#[command(version, about)]
pub struct ArgumentsParser {
    /// The command to run
    #[command(subcommand)]
    pub command: Option<CommandArguments>,
}

impl ArgumentsParser {
    /// Get the [`CommandArguments`] by parsing the arguments.
    ///
    /// Exiting triggers the clap help documentation etc to be displayed.
    #[must_use]
    pub fn get_or_exit() -> CommandArguments {
        match ArgumentsParser::try_parse() {
            Ok(cli) => cli.command.expect("Parsed arguments should have a command"),
            Err(error) => error.exit(),
        }
    }

    /// Get the [`CommandArguments`] by parsing the arguments.
    #[must_use]
    pub fn get() -> Option<CommandArguments> {
        match ArgumentsParser::try_parse().ok() {
            Some(cli) => cli.command,
            None => None,
        }
    }
}
