use crate::parser;
use nom::branch::*;
use nom::bytes::complete::*;
use nom::character::complete::*;
use nom::combinator::*;
use nom::sequence::*;

/// returns a line of input with continued lines folded into a single line
/// strips line ending and continuation characters
pub fn parse_lines(input: &str) -> parser::IResult<&str, &str> {
    //let continued_line_ending = terminated(not_line_ending, preceded(char('\\'), line_ending));
    let line_parser = alt((
        // continued line
        //continued_line_ending,

        // line terminated with \n or \r\n
        terminated(not_line_ending, pair(not(char('\\')), line_ending)),

        // non-terminated line at the end of input
        rest
    ));

    line_parser(input)
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_lines() {
        assert_eq!(parse_lines("abc"), Ok(("", "abc")));
        assert_eq!(parse_lines("abc\n"), Ok(("", "abc")));
        assert_eq!(parse_lines("abc\\\n"), Ok(("", "abc\\")));
        // assert_eq!(parse_lines("abc"), Ok(("", "abc")));
        // assert_eq!(parse_lines("abc\\"), Ok(("", "abc")));
        // assert_eq!(parse_lines("abc\ndef"), Ok(("def", "abc")));
        // assert_eq!(parse_lines("abc\r\ndef"), Ok(("def", "abc")));
    }
}
