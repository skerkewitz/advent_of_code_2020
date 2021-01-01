use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Iter;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;

use tokio::stream::StreamExt;
use tokio::time::delay_queue::Key;

mod test;

fn read_lines_from_file(filename: &str) -> std::io::Result<Vec<String>> {
    let mut f = File::open(filename)?;
    let mut reader = BufReader::new(f);
    return reader.lines().collect();
}

type T = i64;

fn main() -> std::io::Result<()> {
    let mut data = read_lines_from_file("input.txt")?.iter()
        .map(|s| { s.parse::<T>().unwrap() })
        .collect::<Vec<T>>();

    data.sort();

    let mut dp = HashMap::new() as HashMap<T, T>;
    dp.insert(0, 1);
    for &i in data {
        let ans =
            dp.get(&(i - 1)).unwrap_or(&0) +
                dp.get(&(i - 2)).unwrap_or(&0) +
                dp.get(&(i - 3)).unwrap_or(&0);
        println!("I: {} = {}", i, ans);
        dp.insert(i, ans);
    }

    println!("Result: {}", dp[v.last().unwrap()]);

    Ok(())
}
