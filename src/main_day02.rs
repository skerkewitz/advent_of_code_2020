use crate::utils;

use lazy_static::lazy_static;
use regex::{Regex};

lazy_static! { static ref REGEX: Regex = Regex::new(r"^(.*)-(.*) ([a-z]): ([a-z]+)$").unwrap(); }

fn result_part1(data: &Vec<(usize, usize, char, String)>) -> usize {
    data.iter()
        .filter(|(lo, hi, c, pw)| {
            let count = pw.chars().filter(|ch| *ch == *c).count();
            count >= *lo && count <= *hi
        })
        .count()
}

fn result_part2(data: &Vec<(usize, usize, char, String)>) -> usize {
    data.iter()
        .filter(|(lo, hi, c, pw)| {
            let chars = pw.chars().into_iter().collect::<Vec<char>>();
            (chars[lo-1] == *c && chars[hi-1] != *c) || (chars[lo-1] != *c && chars[hi-1] == *c)
        })
        .count()
}

pub fn main() -> std::io::Result<()> {

    let data = utils::read_lines_from_file("inputs/day02/input.txt")?.iter()
        .map(|s|{
            let matches = utils::parse_string_using_regex(s, &REGEX).unwrap();
            let lo = matches[0].parse().unwrap();
            let hi = matches[1].parse().unwrap();
            let c = matches[2].chars().next().unwrap();
            let pw = matches[3].to_string();
            (lo, hi, c, pw)
        })
        .collect::<Vec<(usize, usize, char, String)>>();

    utils::assert_and_print_result("day02, part1", 500, result_part1(&data));
    utils::assert_and_print_result("day02, part2", 313, result_part2(&data));
    Ok(())
}
