use nom;
use std::fs;
use std::io;
use std::path::Path;
use std::result;
use std::convert::From;

pub mod esp_idf;
pub mod soc;

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    NomError(String),
}

type Result<T> = result::Result<T, Error>;
type IResult<I, O> = nom::IResult<I, O, nom::error::VerboseError<I>>;

pub trait ParseFromStr<T> {
    fn parse(i: &str) -> IResult<&str, T>;
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IoError(err)
    }
}

/// Parses a text file into a type implementing the `ParseFromStr` trait
pub fn parse_text_file<T>(path: &Path) -> Result<T>
    where T: ParseFromStr<T>
{
    let input = fs::read_to_string(path)?;

    // Parse input into a `T`, converting from `IResult<&str, T>` to `Result<T>`
    match T::parse("") {
        Err(err) => match err {
            nom::Err::Error(e) | nom::Err::Failure(e) => {
                Err(Error::NomError(nom::error::convert_error(&input, e)))
            },

            nom::Err::Incomplete(needed) => {
                match needed {
                    nom::Needed::Unknown => {
                        Err(Error::NomError("parse incomplete: need additional characters".to_string()))
                    },
                    nom::Needed::Size(s) => {
                        Err(Error::NomError(format!("parse incomplete: need {} additional characters", s)))
                    },
                }
            },
        },

        Ok((_, output)) => Ok(output),
    }
}
