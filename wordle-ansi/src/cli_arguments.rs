/// A command line interface world CLI command.
#[cfg_attr(test, derive(Debug, Eq, PartialEq))]
pub enum WordleCliCommand {
    /// Output this binary vesion.
    Version {
        /// The name of the binary, as it was invoked.
        ///
        /// It is the first argument of the CLI.
        exec: String,
    },
    /// Display the help message.
    Help {
        /// The name of the binary, as it was invoked.
        ///
        /// It is the first argument of the CLI.
        exec: String,
    },
    /// Play wordle game.
    Run {
        /// The name of the binary, as it was invoked.
        ///
        /// It is the first argument of the CLI.
        exec: String,
        /// How words are loaded.
        input: WordleCliInput,
    },
}

impl WordleCliCommand {
    /// Attempt to get the CLI command to run from binary arguments.
    ///
    /// See [std::env::args()].
    pub fn from_args<S: AsRef<str>, I: std::iter::IntoIterator<Item = S>>(
        into_iter: I,
    ) -> Result<Self, WordleCliCommandError> {
        let mut iter = into_iter.into_iter();
        let exec = String::from(
            iter.next()
                .ok_or(WordleCliCommandError::ExecMissing)?
                .as_ref(),
        );
        let next = iter.next();
        let remaining_arguments = iter
            .map(|s| String::from(s.as_ref()))
            .collect::<Vec<String>>();
        match next {
            Some(first_argument) => match first_argument.as_ref() {
                "help" | "--help" | "-h" => {
                    if !remaining_arguments.is_empty() {
                        Err(WordleCliCommandError::UnexpectedArguments {
                            command: "help".into(),
                            arguments: remaining_arguments,
                        })
                    } else {
                        Ok(Self::Help { exec })
                    }
                }
                "version" | "--version" | "-v" => {
                    if !remaining_arguments.is_empty() {
                        Err(WordleCliCommandError::UnexpectedArguments {
                            command: "version".into(),
                            arguments: remaining_arguments,
                        })
                    } else {
                        Ok(Self::Version { exec })
                    }
                }
                _ => {
                    if !remaining_arguments.is_empty() {
                        Err(WordleCliCommandError::UnexpectedArguments {
                            command: "input-file".into(),
                            arguments: remaining_arguments,
                        })
                    } else {
                        Ok(Self::Run {
                            exec,
                            input: WordleCliInput::File(std::path::PathBuf::from(
                                first_argument.as_ref(),
                            )),
                        })
                    }
                }
            },
            None => Ok(Self::Run {
                exec,
                input: WordleCliInput::Stdin,
            }),
        }
    }
}

/// Could not parse wordle CLI command.
#[cfg_attr(test, derive(Debug, Eq, PartialEq))]
pub enum WordleCliCommandError {
    /// The executable name was missing.
    ///
    /// Might happen because providing the executable name/path as the first argument is a
    /// convention.
    ExecMissing,
    /// A command was identified but additional arguments, not expected, were found.
    UnexpectedArguments {
        /// The identified command.
        command: String,
        /// A list a arguments that were not matched.
        arguments: Vec<String>,
    },
}

/// Error while executing wordle command.
pub enum WordleCliExecutionError {
    /// An IO error occurred.
    Io(std::io::Error),
    /// No words were loaded.
    ///
    /// Because no words to guess were found, the Wordle game could not be started.
    NoWords,
}

#[cfg_attr(test, derive(Debug, Eq, PartialEq))]
pub enum WordleCliInput {
    File(std::path::PathBuf),
    Stdin,
}

impl WordleCliInput {
    pub(crate) fn into_reader(self) -> Result<Box<dyn std::io::Read>, std::io::Error> {
        match self {
            WordleCliInput::File(path) => {
                std::fs::File::open(path).map::<Box<dyn std::io::Read>, _>(|file| Box::new(file))
            }
            WordleCliInput::Stdin => Ok(Box::new(std::io::stdin())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{WordleCliCommand, WordleCliCommandError, WordleCliInput};

    #[test]
    fn wordle_cli_command_from_args_exec_missing() {
        let error = WordleCliCommand::from_args::<&str, _>([]);
        assert_eq!(error, Err(WordleCliCommandError::ExecMissing));
    }

    #[test]
    fn wordle_cli_command_from_args_empty_is_run_stdin() {
        let expected_help = WordleCliCommand::from_args(["test exec"]);
        assert_eq!(
            expected_help,
            Ok(WordleCliCommand::Run {
                exec: String::from("test exec"),
                input: WordleCliInput::Stdin
            })
        )
    }

    #[test]
    fn wordle_cli_command_from_args_help() {
        let expected_help = WordleCliCommand::from_args(["test exec", "help"]);
        assert_eq!(
            expected_help,
            Ok(WordleCliCommand::Help {
                exec: String::from("test exec")
            })
        );
        let expected_help = WordleCliCommand::from_args(["test executable name", "--help"]);
        assert_eq!(
            expected_help,
            Ok(WordleCliCommand::Help {
                exec: String::from("test executable name")
            })
        );
        let expected_help = WordleCliCommand::from_args(["test exec", "-h"]);
        assert_eq!(
            expected_help,
            Ok(WordleCliCommand::Help {
                exec: String::from("test exec")
            })
        );
    }

    #[test]
    fn wordle_cli_command_from_args_version() {
        let expected_version = WordleCliCommand::from_args(["test exec", "version"]);
        assert_eq!(
            expected_version,
            Ok(WordleCliCommand::Version {
                exec: String::from("test exec")
            })
        );
        let expected_version = WordleCliCommand::from_args(["test exec", "--version"]);
        assert_eq!(
            expected_version,
            Ok(WordleCliCommand::Version {
                exec: String::from("test exec")
            })
        );
        let expected_version = WordleCliCommand::from_args(["test exec", "-v"]);
        assert_eq!(
            expected_version,
            Ok(WordleCliCommand::Version {
                exec: String::from("test exec")
            })
        );
    }

    #[test]
    fn wordle_cli_command_from_args_single_argument_is_path() {
        let expected_file_input = WordleCliCommand::from_args(["the executable name", "some file"]);
        assert_eq!(
            expected_file_input,
            Ok(WordleCliCommand::Run {
                exec: String::from("the executable name"),
                input: WordleCliInput::File(std::path::PathBuf::from("some file"))
            })
        );
    }

    #[test]
    fn wordle_cli_commands_from_args_unexpected_arguments_help() {
        let expected_error = WordleCliCommand::from_args(["exec name", "help", "arg"]);
        assert_eq!(
            expected_error,
            Err(WordleCliCommandError::UnexpectedArguments {
                command: String::from("help"),
                arguments: vec![String::from("arg")]
            })
        );
        let expected_error = WordleCliCommand::from_args(["exec name", "help", "arg", "ument"]);
        assert_eq!(
            expected_error,
            Err(WordleCliCommandError::UnexpectedArguments {
                command: String::from("help"),
                arguments: vec![String::from("arg"), String::from("ument")]
            })
        );
    }

    #[test]
    fn wordle_cli_commands_from_args_unexpected_arguments_version() {
        let expected_error = WordleCliCommand::from_args(["exec name", "version", "arg"]);
        assert_eq!(
            expected_error,
            Err(WordleCliCommandError::UnexpectedArguments {
                command: String::from("version"),
                arguments: vec![String::from("arg")]
            })
        );
        let expected_error = WordleCliCommand::from_args(["exec name", "version", "arg", "ument"]);
        assert_eq!(
            expected_error,
            Err(WordleCliCommandError::UnexpectedArguments {
                command: String::from("version"),
                arguments: vec![String::from("arg"), String::from("ument")]
            })
        );
    }

    #[test]
    fn wordle_cli_commands_from_args_unexpected_arguments_input_file() {
        let expected_error = WordleCliCommand::from_args(["exec name", "file", "arg"]);
        assert_eq!(
            expected_error,
            Err(WordleCliCommandError::UnexpectedArguments {
                command: String::from("input-file"),
                arguments: vec![String::from("arg")]
            })
        );
        let expected_error =
            WordleCliCommand::from_args(["exec name", "file path", "arg", "ument"]);
        assert_eq!(
            expected_error,
            Err(WordleCliCommandError::UnexpectedArguments {
                command: String::from("input-file"),
                arguments: vec![String::from("arg"), String::from("ument")]
            })
        );
    }
}
