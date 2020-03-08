//! Parses .c and .h files like a C-preprocessor and extract directives like #define
//! This is just good enough to extract information from the ESP-IDF source and does not
//! attempt to be complete or even entirely correct as parsing arbitrary C code is difficult.
//!
//! C-preprocessor ref: https://gcc.gnu.org/onlinedocs/gcc-2.95.3/cpp_1.html

use regex::Regex;
use std::fs::File;
use std::io::Read;

/// Data extracted from .c and .h files
pub enum CppData {
    /// No data
    Empty,

    /// Skipped data
    Unknown { line: String },

    /// #ifdef
    IfDef { name: String },

    /// #define
    Define {
        name: String,
        arguments: Vec<String>,
        body: String,
    },
}

lazy_static! {
    /// Matches a line comment to end of line or end of file
    static ref REGEX_LINE_COMMENT: Regex = Regex::new("//.*?\n").unwrap();

    /// Matches a block comment
    static ref REGEX_BLOCK_COMMENT: Regex = Regex::new("(?s)/\\*.*?\\*/").unwrap();
}

/// Parses a C .h and .c files
pub fn read_cpp(path: &str) {
    let text = cpp_transform(&read_file(path));
    print!("{}", text);
}

/// Reads a file at `path` and returns contents as a String
fn read_file(path: &str) -> String {
    let mut f = File::open(path).unwrap();
    let mut text = String::new();
    f.read_to_string(&mut text).unwrap();
    text
}

/// Applies C-preprocessor global transformations
/// Known issues:
///  * Incorrectly removes comments from inside of string-literals
fn cpp_transform(input: &str) -> String {
    // Ensure text ends with a newline
    let mut text = match input.ends_with("\n") {
        true => input.to_string(),
        false => format!("{}\n", input),
    };

    // Continue lines ending with Backslash-Newline
    text = text.replace("\\\n", "");

    // Replace line comments with space followed by newline
    text = REGEX_LINE_COMMENT.replace_all(&text, " \n").to_string();

    // Replace block comments with single space
    text = REGEX_BLOCK_COMMENT.replace_all(&text, " ").to_string();

    text
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    /// Test line continuation
    fn test_line_continuation() {
        assert_eq!(cpp_transform("abc"), "abc\n");
        assert_eq!(cpp_transform("abc\n"), "abc\n");
        assert_eq!(cpp_transform("abc\ndef"), "abc\ndef\n");
        assert_eq!(cpp_transform("abc\\\ndef"), "abcdef\n");
    }

    #[test]
    /// Test line comments
    fn test_line_comments() {
        assert_eq!(cpp_transform("//abc\n"), " \n");
        assert_eq!(cpp_transform("//abc\n//def\n"), " \n \n");
        assert_eq!(cpp_transform("//abc"), " \n");
    }

    #[test]
    /// Test block comments
    fn test_block_comments() {
        assert_eq!(cpp_transform("/*abc*/"), " \n");
        assert_eq!(cpp_transform("/*abc*/\n/*def*/"), " \n \n");
        assert_eq!(cpp_transform("/**abc*/"), " \n");
        assert_eq!(cpp_transform("/**\n * abc\n */"), " \n");
    }
}
