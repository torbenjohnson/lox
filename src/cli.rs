use std::path::PathBuf;

#[derive(thiserror::Error, displaydoc::Display, Debug, PartialEq, Eq)]
/// Errors that can occur while parsing CLI arguments.
pub enum CliError {
    /// More than one script path was provided. Usage: `lox [script]`.
    TooManyArguments,
}

#[derive(Debug, PartialEq, Eq)]
/// Parsed representation of command-line arguments.
pub struct Cli {
    input_file_path: Option<PathBuf>,
}

impl Cli {
    /// Parse command-line arguments into a [`Cli`].
    ///
    /// # Errors
    ///
    /// Returns [`CliError::TooManyArguments`] if more than one script path is supplied.
    pub fn parse<E: ExactSizeIterator<Item = String>>(mut args: E) -> Result<Self, CliError> {
        if args.len() > 2 {
            return Err(CliError::TooManyArguments);
        }

        // Skip the binary name (index 0) and read the optional script path (index 1).
        let input_file_path_arg = args.nth(1);
        let input_file_path = input_file_path_arg.map(PathBuf::from);

        Ok(Self { input_file_path })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_too_many_arg_when_parse_then_too_many_arg_error() {
        let input = [
            String::from("test"),
            String::from("test"),
            String::from("test"),
        ]
        .into_iter();

        let result = Cli::parse(input);

        assert_eq!(result, Err(CliError::TooManyArguments));
    }

    #[test]
    fn given_agument_when_parse_then_path() {
        let input = [String::from("test"), String::from("test")].into_iter();

        let result = Cli::parse(input).unwrap();

        assert_eq!(
            result,
            Cli {
                input_file_path: Some(PathBuf::from("test"))
            }
        );
    }
}
