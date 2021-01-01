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

type T = u64;
const len: usize =  25;

fn contains_sum(data: &[T], number_to_check: T) -> bool {

    for x in 0..data.len() - 1 {
        for y in x+1..data.len() {
            let sum = data[x] + data[y];
            if sum == number_to_check {
                //println!("Hit at {}, {} = {}", x, y, sum);
                return true;
            }
        }
    }

    return false
}

fn find_first_mismatch(data: &Vec<T>) -> Result<T, &str> {

    for i in 0..data.len() - (len  + 1) {

        let number_to_check = data[i+len];

        let subset = &data.as_slice()[i..i+len];

        let hit = contains_sum(subset, number_to_check);
        if hit == true { continue };

        println!("No hit at start index {} for {} index {}", i, number_to_check, i + len);
        return Ok(number_to_check)
    }

    return Err("No mismatch found")
}

fn main() -> std::io::Result<()> {

    let data = read_lines_from_file("sample.txt")?.iter()
        .map(|s| { s.parse::<T>().unwrap() })
        .enumerate()
        .inspect(|s| println!("org at {}: {}", s.0, s.1))
        .map(|s| { s.1})
        .collect::<Vec<T>>();


    let first_mismatch = match find_first_mismatch(&data) {
        Ok(n) => n,
        Err(err) => panic!("Could not find mismatch: {}", err)
    };

    let mut lo = 0;
    let mut hi = 0;

    for i in 0..data.len() - 1 {

        for j in i+1..data.len() {

            let subset = &data.as_slice()[i..j];
            let sum  = subset
                .iter()
                .sum::<u64>();
            if sum > first_mismatch {
                break;
            }

            if sum == first_mismatch {
                let min = subset.iter().min().unwrap();
                let max = subset.iter().max().unwrap();

                println!("Match is {} + {} = {} (len) {} sum {}", i, j-1, i+j, j-1, sum);
                println!("Min {}, max {} = {}", min, max, min + max);
                //return Ok(());
            }
        }
    }



    Ok(())
}
