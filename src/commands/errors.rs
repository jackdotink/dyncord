use thiserror::Error;

use crate::commands::arguments::ParsingError;

#[derive(Debug, Error)]
pub enum CommandError {
    #[error("An error occurred while parsing the command's arguments: {0}")]
    Parsing(#[from] ParsingError),
}
