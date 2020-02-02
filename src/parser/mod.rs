use nom;
use std::fs;
use std::io;
use std::path::Path;
use std::result;
use std::convert::From;

pub mod cpp;
pub mod esp_idf;
pub mod soc;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Parse,
}

type Result<T> = result::Result<T, Error>;
type IResult<I, O> = nom::IResult<I, O, nom::error::VerboseError<I>>;

pub trait ParseFromStr<T> {
    fn parse(i: &str) -> IResult<&str, T>;
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}

/// Parses a text file into a type implementing the `ParseFromStr` trait
pub fn parse_text_file<T>(path: &Path) -> Result<T>
    where T: ParseFromStr<T>
{
    let input = fs::read_to_string(path)?;
    //println!("{}", input);

    // Parse input into a `T`, converting from `IResult<&str, T>` to `Result<T>`
    match T::parse(&input) {
        Err(err) => match err {
            nom::Err::Error(e) | nom::Err::Failure(e) => {
                println!("Parse Error:\n{}", nom::error::convert_error(&input, e));
                Err(Error::Parse)
            },

            nom::Err::Incomplete(needed) => {
                match needed {
                    nom::Needed::Unknown => {
                        println!("parse incomplete: need additional characters");
                        Err(Error::Parse)
                    },
                    nom::Needed::Size(s) => {
                        println!("parse incomplete: need {} additional characters", s);
                        Err(Error::Parse)
                    },
                }
            },
        },

        Ok((_, output)) => Ok(output),
    }
}
