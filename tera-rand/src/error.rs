use anyhow::anyhow;
use tera::Error;
use thiserror::Error;

#[derive(Error, Debug)]
enum TeraRandError {
    #[error("Unable to parse argument for `{parameter}` in function `{function} due to error`")]
    UnableToParseArgument {
        function: &'static str,
        parameter: &'static str,
        source: anyhow::Error,
    },
    #[error("Unsupported argument `{argument}` for `{parameter}` in function `{function}`")]
    UnsupportedArgument {
        function: &'static str,
        parameter: &'static str,
        argument: String,
    },
    #[error("Required argument missing for parameter `{parameter}` in function `{function}`")]
    RequiredArgumentMissing {
        function: &'static str,
        parameter: &'static str,
    },
    #[error("Unable to read file at path: `{path}`")]
    UnableToReadFile {
        function: &'static str,
        path: String,
        source: anyhow::Error,
    },
    #[error("Unable to sample from an empty file: `{path}`")]
    EmptyFile {
        function: &'static str,
        path: String,
    },
    #[error("Internal error: {0}")]
    Internal(String),
}

// Tera functions must return a `Result` using `tera::Error`, so
// we need to convert our internal errors
impl Into<tera::Error> for TeraRandError {
    fn into(self) -> Error {
        match &self {
            Self::UnableToParseArgument {
                function,
                parameter: _parameter,
                source: _source,
            } => tera::Error::call_function(*function, self),
            Self::UnsupportedArgument {
                function,
                parameter: _parameter,
                argument: _argument,
            } => tera::Error::call_function(*function, self),
            Self::RequiredArgumentMissing {
                function,
                parameter: _parameter,
            } => tera::Error::call_function(*function, self),
            Self::UnableToReadFile {
                function,
                path: _path,
                source: _source,
            } => tera::Error::call_function(*function, self),
            Self::EmptyFile {
                function,
                path: _path,
            } => tera::Error::call_function(*function, self),
            Self::Internal(_msg) => tera::Error::from(self.to_string()),
        }
    }
}

// convenience

pub(crate) fn arg_parse_error(
    function: &'static str,
    parameter: &'static str,
    source: impl Into<anyhow::Error>,
) -> tera::Error {
    let tera_rand_error: TeraRandError = TeraRandError::UnableToParseArgument {
        function,
        parameter,
        source: anyhow!(source),
    };
    Into::<tera::Error>::into(tera_rand_error)
}

pub(crate) fn unsupported_arg(
    function: &'static str,
    parameter: &'static str,
    argument: String,
) -> tera::Error {
    let tera_rand_error: TeraRandError = TeraRandError::UnsupportedArgument {
        function,
        parameter,
        argument,
    };
    Into::<tera::Error>::into(tera_rand_error)
}

pub(crate) fn missing_arg(function: &'static str, parameter: &'static str) -> tera::Error {
    let tera_rand_error: TeraRandError = TeraRandError::RequiredArgumentMissing {
        function,
        parameter,
    };
    Into::<tera::Error>::into(tera_rand_error)
}

pub(crate) fn read_file_error(
    function: &'static str,
    path: String,
    source: impl Into<anyhow::Error>,
) -> tera::Error {
    let tera_rand_error: TeraRandError = TeraRandError::UnableToReadFile {
        function,
        path,
        source: anyhow!(source),
    };
    Into::<tera::Error>::into(tera_rand_error)
}

pub(crate) fn empty_file(function: &'static str, path: String) -> tera::Error {
    let tera_rand_error: TeraRandError = TeraRandError::EmptyFile { function, path };
    Into::<tera::Error>::into(tera_rand_error)
}

pub(crate) fn internal_error(msg: String) -> tera::Error {
    let tera_rand_error: TeraRandError = TeraRandError::Internal(msg);
    Into::<tera::Error>::into(tera_rand_error)
}
