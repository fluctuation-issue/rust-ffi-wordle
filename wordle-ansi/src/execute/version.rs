pub fn write_version<W: std::io::Write>(mut writer: W, executable_name: &str) {
    writeln!(
        &mut writer,
        "{}",
        generate_version_output(executable_name, env!("CARGO_PKG_VERSION"))
    )
    .expect("failed to write version");
}

fn generate_version_output(executable_name: &str, version: &str) -> String {
    if executable_name.is_empty() {
        panic!("executable name must not be empty");
    }
    if version.is_empty() {
        panic!("version must not be empty");
    }
    format!("{} version {}", executable_name, version)
}

#[cfg(test)]
mod tests {
    use super::generate_version_output;

    #[test]
    #[should_panic]
    fn generate_version_output_executable_name_empty_panics() {
        generate_version_output("", "0.1.0");
    }

    #[test]
    #[should_panic]
    fn generate_version_output_version_empty_panics() {
        generate_version_output("wordle-ansi", "");
    }

    #[test]
    fn test_generate_version_output() {
        assert_eq!(
            &generate_version_output("exec name", "1.0 alpha"),
            "exec name version 1.0 alpha"
        );
    }
}
