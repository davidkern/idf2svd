use nom::{
    IResult,
};
use crate::parsing::ParseFrom;

pub struct RegAddr;

impl<'a> ParseFrom<&'a str> for RegAddr {
    fn parse_from(input: &'a str) -> IResult<&'a str, Self> {
        let reg_addr = RegAddr{ };

        Ok((input, reg_addr))
    }
}
