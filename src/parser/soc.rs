use std::fs;
use nom::{
    bytes::complete::{tag},
};
use crate::parser;

#[derive(Default, Debug, Clone)]
pub struct Soc {

}

impl parser::ParseFromStr<Soc> for Soc {
    fn parse(input: &str) -> parser::IResult<&str, Self> {
        let (input, _) = tag("#")(input)?;

        Ok((input, Soc{}))
    }
}
