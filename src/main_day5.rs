mod test;

use std::fs::File;
use std::io::{BufReader, BufRead};
use std::fmt;
use std::collections::HashSet;
use tokio::time::delay_queue::Key;

fn read_lines_from_file(filename: &str) -> std::io::Result<Vec<String>> {
    let mut f = File::open(filename)?;
    let mut reader = BufReader::new(f);
    return reader.lines().collect()
}


fn calc_seat_row(input: &str) -> u32 {
    let mut lo = 0;
    let mut hi = 128;

    for c in input.chars() {
        match c {
            'F' => hi -= (hi - lo) / 2,
            'B' => lo += (hi - lo) / 2,
            _ => panic!("Unknown chart {}", c)
        }
    }

    return hi - 1;
}

fn calc_seat_column(input: &str) -> u32 {
    let mut lo = 0;
    let mut hi = 8;

    for c in input.chars() {
        match c {
            'L' => hi -= (hi - lo) / 2,
            'R' => lo += (hi - lo) / 2,
            _ => panic!("Unknown chart {}", c)
        }
    }

    return hi - 1;
}

fn calc_seat_id(input: &str) -> (u32, u32) {
    let split = input.split_at(7);

    assert!(split.0.chars().all(|c| c == 'F' || c == 'B'));
    assert_eq!(split.0.len(), 7);
    assert!(split.1.chars().all(|c| c == 'L' || c == 'R'));
    assert_eq!(split.1.len(), 3);

    return (calc_seat_row(split.0), calc_seat_column(split.1));
}

fn calc_unique_seat_id(seat_id: (u32, u32)) -> u32 {
    return seat_id.0 * 8 + seat_id.1;
}

fn main() -> std::io::Result<()> {
    // let ref_set: HashSet<String> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid" /*, "cid"*/]
    //     .iter()
    //     .map(|s| String::from(*s))
    //     .collect();


    let mut lines = read_lines_from_file("input.txt")?
        .iter()
        .map(|a| calc_seat_id(a))
        .map(|a| calc_unique_seat_id(a))
        .collect::<Vec<u32>>();





    lines.sort();

    let offset = lines[0];
    for (i, seat_id) in lines.iter().enumerate() {
        if i as u32 + offset != *seat_id {
            println!("Your seat is: {}", seat_id);
        }
    }



        //     // .inspect(|x| println!("One: {}", x))
        //     .map(|a| parse_keys(&a).iter().map(|a| a.key.clone()).collect::<HashSet<String>>())
        //     .map(|a| a.intersection(&ref_set).count() == 7)
        //     .filter(|a| *a)
        //     .count();
        //        .collect::<Vec<u32>>();


    // let input = "FFFBBBFRRR";
    //
    // let seat_id = calc_seat_id(input);


    //println!("Result is: {}", lines);
    //println!("Result is: row {} column {} id {}", seat_id.0, seat_id.1, seat_id.0 * 8 + seat_id.1);

    // for s in lines {
    //     println!("--- {}", s);
    //     // for kv in s {
    //     //     println!(" .. kv {}", kv);
    //     // }
    //
    // }

    // let tree_slope4 = count_trees(&data, 7, 1);
    // let tree_slope5 = count_trees(&data, 1, 2);
    //
    // println!("Tree is {}", tree_slope1 * tree_slope2 * tree_slope3 * tree_slope4 * tree_slope5);

    Ok(())
}
