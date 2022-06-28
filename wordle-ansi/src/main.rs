use wordle_ansi::{execute, WordleCliCommand, WordleCliCommandError, WordleCliExecutionError};

fn display_command_error(wordle_command_error: WordleCliCommandError) -> std::process::ExitCode {
    match wordle_command_error {
        WordleCliCommandError::ExecMissing => {
            eprintln!("fatal error: could not retrieve executable name")
        }
        WordleCliCommandError::UnexpectedArguments { command, arguments } => {
            eprintln!(
                    "Did not expect arguments `{}` for command `{}`.\nRun `wordle-ansi help` for usage.",
                    arguments.join(","),
                    command
                );
        }
    }
    std::process::ExitCode::FAILURE
}

fn perfom_command(command: WordleCliCommand) -> std::process::ExitCode {
    if let Err(execution_error) = execute(command) {
        match execution_error {
            WordleCliExecutionError::Io(io_error) => eprintln!("io error: {}", io_error),
            WordleCliExecutionError::NoWords => eprintln!("provided file did not contain any word"),
        }
        return std::process::ExitCode::FAILURE;
    }
    std::process::ExitCode::SUCCESS
}

fn main() -> std::process::ExitCode {
    let command = WordleCliCommand::from_args(std::env::args());
    match command {
        Err(wordle_command_error) => display_command_error(wordle_command_error),
        Ok(command) => perfom_command(command),
    }
}
