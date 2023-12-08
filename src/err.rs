use thiserror::Error;

#[derive(Error, Debug)]
pub enum InputError {
    #[error("Invalid input: {0}")]
    WrongFormat(String),
}