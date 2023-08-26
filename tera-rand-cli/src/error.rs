use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum TeraRandCliError {
    #[error(
        "Either both or neither of `batch_size` and `batch_interval` should be included. \
         It is an error to include only one of the two."
    )]
    InvalidBatchArguments,
}
