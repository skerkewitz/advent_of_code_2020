use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Iter;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;


mod test;

fn read_lines_from_file(filename: &str) -> std::io::Result<Vec<String>> {
    let mut f = File::open(filename)?;
    let mut reader = BufReader::new(f);
    return reader.lines().collect();
}

type T = char;

const EMPTY_SEAT: T = 'L';
const OCCUPIED_SEAT: T = '#';
const FLOOR: T = '.';

type M = Vec<Vec<char>>;

fn print_map(data: &M) {
    for s in data.as_slice() {
        println!("{}", String::from_iter(s.iter().clone()));
    }
}

fn is_seat_type(data: &M, (x, y): (i32, i32), seat_type: char) -> bool {
    if x < 0 || y < 0 {
        return false;
    }

    let xu = x as usize;
    let yu = y as usize;


    if yu > (data.len() - 1) || xu > (data[yu].len() - 1) {
        return false;
    }

    data[yu][xu] == seat_type
}

fn cast_ray(data: &M, (px, py): (usize, usize), (dx, dy): (i32, i32)) -> Option<char> {

    if px as i32+ dx < 0 || py as i32 + dy < 0 {
        return None
    }

    let xu = (px as i32 + dx) as usize;
    let yu = (py as i32 + dy) as usize;

    if yu > (data.len() - 1) || xu > (data[yu].len() - 1) {
        return None
    }

    let v = data[yu][xu];
    if v == FLOOR {
        return cast_ray(data, (xu, yu), (dx, dy))
    }

    return Some(v)
}

fn count_adjacent(data: &M, (x, y): (usize, usize), seat_type: char) -> usize {
    let ux = x as i32;
    let uy = y as i32;


    let mut seat_ids: Vec<(i32, i32)> = vec![
        (ux - 1, uy - 1), (ux, uy - 1), (ux + 1, uy - 1),
        (ux - 1, uy), /*(x,y)*/ (ux + 1, uy),
        (ux - 1, uy + 1), (ux, uy + 1), (ux + 1, uy + 1)
    ];

    return seat_ids.iter()
        .filter(|id| is_seat_type(data, **id, seat_type))
        .count();
}

fn count_adjacent_ray(data: &M, (x, y): (usize, usize), seat_type: char) -> usize {
    let ux = x as i32;
    let uy = y as i32;


    let mut seat_ids: Vec<(i32, i32)> = vec![
        (-1, -1), (0, -1), (1, -1),
        (-1, 0), /*(x,y)*/ (1, 0),
        (-1, 1), (0, 1), (1, 1)
    ];

    return seat_ids.iter()
        .filter(|id| {
            match cast_ray(data, (x,y), **id) {
                Some(n) if n == seat_type => true,
                _ => false
            }
        })
        .count();
}

fn run(data: &M) -> M {
    return data.iter()
        .enumerate()
        .map(|(y, iv)| {
            iv.iter()
                .enumerate()
                .map(|(x, c)| {
                    return match *c {
                        FLOOR => FLOOR,
                        EMPTY_SEAT => {
                            if count_adjacent_ray(&data, (x, y), OCCUPIED_SEAT) == 0 {
                                OCCUPIED_SEAT
                            } else {
                                EMPTY_SEAT
                            }
                        }
                        OCCUPIED_SEAT => {
                            if count_adjacent_ray(&data, (x, y), OCCUPIED_SEAT) >= 5 {
                                EMPTY_SEAT
                            } else {
                                OCCUPIED_SEAT
                            }
                        }
                        _ => unreachable!()
                    };
                })
                .collect()
        })
        .collect();
}

fn eq(lhs: &M, rhs: &M) -> bool {
    return lhs.iter()
        .zip(rhs.iter())
        .all(|(liv, riv)| {
            liv.iter()
                .zip(riv.iter())
                .all(|(lx, rx)| *lx == *rx) == true
        });

}

fn main() -> std::io::Result<()> {
    let mut data = read_lines_from_file("sample.txt")?.iter()
        .map(|s| { s.chars().into_iter().map(|c| c).collect::<Vec<char>>() })
        .collect::<Vec<Vec<char>>>();

    print_map(&data);

    println!();

    println!("{}", count_adjacent(&data, (0, 0), 'L'));

    let mut m = data;

    let mut i = 0;
    let mut did_change = true;
    while did_change {
        let nm = run(&m);
        i += 1;
        println!("Gen {}", i);
        println!();
        print_map(&m);
        println!();
        did_change = !eq(&nm, &m);
        println!("Did change {}", did_change);
        println!();
        m = nm;
    }

    let result = m.iter()
        .map(|iv| iv.iter().filter(|c| **c == OCCUPIED_SEAT).count())
        .sum::<usize>();

    println!("Result: {}", result);

    Ok(())
}
