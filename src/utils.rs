use std::io::{BufReader, BufRead, Error};
use std::fs::File;
use std::str::FromStr;
use std::fmt::{Debug, Display};
use regex::Regex;

/// Read a lines from the given file name
pub fn read_lines_from_file(filename: &str) -> std::io::Result<Vec<String>> {
    return BufReader::new(File::open(filename)?).lines().collect();
}

/// Converts the given string to a number.
#[inline]
pub(crate) fn string_to_number<T: FromStr>(s: &String) -> T where <T as FromStr>::Err: Debug {
    s.parse().unwrap()
}

/// Parse the given string using the given Regex into a Vec of strings
pub fn parse_string_using_regex(s: &str, regex: &Regex) -> Option<Vec<String>> {
    if let Some(captures) = regex.captures(s) {
        let vec = captures.iter()
            .skip(1)
            .map(|x| x.unwrap().as_str().to_string())
            .collect();
        return Some(vec)
    }

    None
}

pub(crate) fn assert_and_print_result<T: Debug + Display + PartialEq>(s: &str, expected: T, actual: T) {
    assert_eq!(expected, actual);
    println!("Result {}: {}", s, expected);
}