use std::io;
use std::result;
use std::convert::From;

pub mod esp_idf;
pub mod soc;

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
}

type Result<T> = result::Result<T, Error>;

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IoError(err)
    }
}
