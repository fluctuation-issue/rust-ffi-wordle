//! ANSI front for Wordle.

#![deny(missing_docs)]

mod ansi;
mod cli_arguments;
mod execute;

pub use cli_arguments::{WordleCliCommand, WordleCliCommandError, WordleCliExecutionError};

/// Attempt to execute the given cli command.
pub fn execute(command: WordleCliCommand) -> Result<(), WordleCliExecutionError> {
    match command {
        WordleCliCommand::Version { exec } => {
            execute::write_version(std::io::stdout(), &exec);
            Ok(())
        }
        WordleCliCommand::Help { exec } => {
            execute::write_help(std::io::stdout(), &exec);
            Ok(())
        }
        WordleCliCommand::Run { exec: _, input } => execute::run_game(input),
    }
}
