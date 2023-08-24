use anyhow::anyhow;
use tera::Error;
use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum TeraRandError {
    #[error("Unable to parse argument for `{0}` due to error")]
    UnableToParseArgument(&'static str, #[source] anyhow::Error),

    #[error("Unsupported argument `{argument}` for `{parameter}`")]
    UnsupportedArgument {
        parameter: &'static str,
        argument: String,
    },

    #[error("Required argument missing for parameter `{0}`")]
    RequiredArgumentMissing(&'static str),

    #[error("Unable to read file at path: `{0}`")]
    UnableToReadFile(String, #[source] anyhow::Error),

    #[error("Unable to sample from an empty file: `{0}`")]
    EmptyFile(String),

    #[error(
        "Provided cidr length {provided_bound}, which is out of bounds. \
         Cidr length should be between {valid_bound_start} and {valid_bound_end}"
    )]
    CidrPrefixLengthOutOfBounds {
        provided_bound: u32,
        valid_bound_start: u32,
        valid_bound_end: u32,
    },

    #[error("Internal error: {0}")]
    Internal(String),
}

// Tera functions must return a `Result` using `tera::Error`, so
// we need to convert our internal errors
impl Into<tera::Error> for TeraRandError {
    fn into(self) -> Error {
        match &self {
            Self::UnableToParseArgument(_parameter, _source) => tera::Error::msg(self),
            Self::UnsupportedArgument {
                parameter: _parameter,
                argument: _argument,
            } => tera::Error::msg(self),
            Self::RequiredArgumentMissing(_parameter) => tera::Error::msg(self),
            Self::UnableToReadFile(_path, _source) => tera::Error::msg(self),
            Self::EmptyFile(_path) => tera::Error::msg(self),
            Self::CidrPrefixLengthOutOfBounds {
                provided_bound: _provided_bound,
                valid_bound_start: _valid_bound_start,
                valid_bound_end: _valid_bound_end,
            } => tera::Error::msg(self),
            Self::Internal(_msg) => tera::Error::msg(self.to_string()),
        }
    }
}

// convenience

pub(crate) fn arg_parse_error(
    parameter: &'static str,
    source: impl Into<anyhow::Error>,
) -> tera::Error {
    let tera_rand_error: TeraRandError =
        TeraRandError::UnableToParseArgument(parameter, anyhow!(source));
    Into::<tera::Error>::into(tera_rand_error)
}

pub(crate) fn unsupported_arg(parameter: &'static str, argument: String) -> tera::Error {
    let tera_rand_error: TeraRandError = TeraRandError::UnsupportedArgument {
        parameter,
        argument,
    };
    Into::<tera::Error>::into(tera_rand_error)
}

pub(crate) fn missing_arg(parameter: &'static str) -> tera::Error {
    let tera_rand_error: TeraRandError = TeraRandError::RequiredArgumentMissing(parameter);
    Into::<tera::Error>::into(tera_rand_error)
}

pub(crate) fn read_file_error(path: String, source: impl Into<anyhow::Error>) -> tera::Error {
    let tera_rand_error: TeraRandError = TeraRandError::UnableToReadFile(path, anyhow!(source));
    Into::<tera::Error>::into(tera_rand_error)
}

pub(crate) fn empty_file(path: String) -> tera::Error {
    let tera_rand_error: TeraRandError = TeraRandError::EmptyFile(path);
    Into::<tera::Error>::into(tera_rand_error)
}

pub(crate) fn cidr_prefix_length_out_of_bounds(
    provided_bound: u32,
    valid_bound_start: u32,
    valid_bound_end: u32,
) -> tera::Error {
    let tera_rand_error: TeraRandError = TeraRandError::CidrPrefixLengthOutOfBounds {
        provided_bound,
        valid_bound_start,
        valid_bound_end,
    };
    Into::<tera::Error>::into(tera_rand_error)
}

pub(crate) fn internal_error(msg: String) -> tera::Error {
    let tera_rand_error: TeraRandError = TeraRandError::Internal(msg);
    Into::<tera::Error>::into(tera_rand_error)
}
