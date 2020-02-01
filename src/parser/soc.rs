use std::fs;
use nom;
use crate::parser;

#[derive(Default, Debug, Clone)]
pub struct Soc {

}

impl parser::ParseFromStr<Soc> for Soc {
    fn parse(i: &str) -> parser::IResult<&str, Self> {
        Ok(("", Soc{}))
    }
}
