pub fn write_help<W: std::io::Write>(mut writer: W, executable_name: &str) {
    writeln!(
        &mut writer,
        concat!(
            r#"{}
    Play wordle in the terminal.
    
SYNOPSIS
    {} version      Display the binary version.
    {} --version    Same as above.
    {} -v           Same as above.
    {} help         Display this help message.
    {} --help       Same as above.
    {} -h           Same as above.
    {} [file path]  Play wordle picking a random word."#,
            " See the \x1b[1mGAME\x1b[0m section.",
            r#"

GAME
    The word is picked from the input file, randomly.
    The file is expected to contain one word per line.
    Line terminator is "#,
            "\x1b[4m'\\n'\x1b[0m.",
            r#"
    Empty lines are discarded.

    If no files are specified, then read words from "#,
            "\x1b[1mSTDIN\x1b[0m."
        ),
        executable_name,
        executable_name,
        executable_name,
        executable_name,
        executable_name,
        executable_name,
        executable_name,
        executable_name
    )
    .expect("failed to write help");
}
