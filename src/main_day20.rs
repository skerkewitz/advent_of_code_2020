use std::{f32};
use std::collections::HashSet;
use std::fmt::{Debug, Display};
use std::fs::File;
use std::io::{BufRead, BufReader};

use lazy_static::lazy_static;
use regex::Regex;

fn read_lines_from_file(filename: &str) -> std::io::Result<Vec<String>> {
    let f = File::open(filename)?;
    let reader = BufReader::new(f);
    return reader.lines().collect();
}

type I = usize;
type PartId = I;

lazy_static! {
 static ref RE: Regex = Regex::new(r"^Tile (\d+):$").unwrap();
}

const EDGE_LEN: usize = 10;

trait FlipAndRotate {
    fn turned_right(&self) -> Self;
    fn flipped_horizontal(&self) -> Self;
}

trait Permutate<T> {
    fn permutation(&self) -> Vec<T>;
}

impl<T: Clone> FlipAndRotate for Vec<T> {
    fn turned_right(&self) -> Self {
        let mut turned_data = Vec::with_capacity(self.len()) as Vec<T>;

        let edge_len = (self.len() as f32).sqrt() as usize;
        (0..edge_len).for_each(|x| {
            (0..edge_len).for_each(|y| {
                turned_data.push(self[(x + ((edge_len - 1) - y) * edge_len)].clone());
            })
        });
        turned_data
    }

    fn flipped_horizontal(&self) -> Self {
        let mut turned_data = Vec::with_capacity(self.len()) as Vec<T>;
        let edge_len = (self.len() as f32).sqrt() as usize;
        (0..edge_len).rev().for_each(|y| {
            (0..edge_len).for_each(|x| {
                turned_data.push(self[x + (y * edge_len)].clone());
            })
        });
        turned_data
    }
}

trait Print {
    fn print(&self, edge_len: usize);
}

impl<T: Debug + Display> Print for Vec<T> {
    fn print(&self, edge_len: usize) {
        (0..edge_len).for_each(|y| {
            (0..edge_len).for_each(|x| {
                print!("{}", self[x + (y * edge_len)]);
            });
            println!()
        });
    }
}

#[derive(Debug, Clone)]
struct Part {
    tile_id: I,
    data: Vec<bool>,
}

#[derive(Debug, Clone)]
enum Edge {
    Top,
    Left,
    Right,
    Bottom,
}

impl FlipAndRotate for Part {

    fn turned_right(&self) -> Self {
        Part {
            tile_id: self.tile_id,
            data: self.data.turned_right(),
        }
    }

    fn flipped_horizontal(&self) -> Self {
        Part {
            tile_id: self.tile_id,
            data: self.data.flipped_horizontal(),
        }
    }
}

impl<T: Clone + FlipAndRotate> Permutate<T> for T {

    fn permutation(&self) -> Vec<T> {
        let mut permutation = Vec::with_capacity(8) as Vec<T>;

        permutation.push(self.clone());
        let part = self.turned_right();
        permutation.push(part.clone());
        let part = part.turned_right();
        permutation.push(part.clone());
        let part = part.turned_right();
        permutation.push(part.clone());

        let flipped = self.flipped_horizontal();
        permutation.push(flipped.clone());
        let flipped = flipped.turned_right();
        permutation.push(flipped.clone());
        let flipped = flipped.turned_right();
        permutation.push(flipped.clone());
        let flipped = flipped.turned_right();
        permutation.push(flipped.clone());

        assert_eq!(permutation.len(), 8);
        permutation
    }
}

impl Part {
    fn parse_part_id(s: &String) -> Option<I> {
        Some(RE.captures(s)?.get(1)?.as_str().parse::<PartId>().ok()?)
    }

    fn from(v: Vec<String>) -> Self {
        let mut iter = v.iter();
        let tile_str = iter.next().expect("Tile Id");
        let tile_id = Part::parse_part_id(tile_str).unwrap();
        let vec_data = iter.map(|x| x.chars().map(|c| c == '#').collect::<Vec<bool>>())
            .flatten()
            .collect::<Vec<bool>>();

        Part {
            tile_id,
            data: vec_data,
        }
    }

    fn print(&self) {
        println!("Tile {}", self.tile_id);
        (0..EDGE_LEN).for_each(|y| {
            (0..EDGE_LEN).for_each(|x| {
                print!("{}", self.get_at(x, y));
            });
            println!()
        });
    }

    fn get_at(&self, x: usize, y: usize) -> char {
        if *self.data.get(y * EDGE_LEN + x).unwrap() { '#' } else { '.' }
    }

    fn edge(&self, edge: &Edge) -> String {
        match edge {
            Edge::Top => self.top_edge(),
            Edge::Left => self.left_edge(),
            Edge::Right => self.right_edge(),
            Edge::Bottom => self.bottom_edge()
        }
    }

    fn opposite_edge(&self, edge: &Edge) -> String {
        match edge {
            Edge::Top => self.bottom_edge(),
            Edge::Left => self.right_edge(),
            Edge::Right => self.left_edge(),
            Edge::Bottom => self.top_edge()
        }
    }

    fn left_edge(&self) -> String {
        (0..EDGE_LEN).map(|y| self.get_at(0, y)).collect::<String>()
    }

    fn right_edge(&self) -> String {
        (0..EDGE_LEN).map(|y| self.get_at(EDGE_LEN - 1, y)).collect::<String>()
    }

    fn top_edge(&self) -> String {
        (0..EDGE_LEN).map(|x| self.get_at(x, 0)).collect::<String>()
    }

    fn bottom_edge(&self) -> String {
        (0..EDGE_LEN).map(|x| self.get_at(x, EDGE_LEN - 1)).collect::<String>()
    }
}

trait DeletePart {
    fn delete_part(&mut self, p: &Part) -> bool;
}

impl DeletePart for Vec<Part> {
    fn delete_part(&mut self, p: &Part) -> bool {
        if let Some(n) = self.iter().position(|x| x.tile_id == p.tile_id) {
            self.remove(n);
            return true;
        }

        return false;
    }
}

fn find_part_with_no_edge(edge: &Edge, current: &Part, parts: &Vec<Part>) -> Part {
    for p in parts {
        if p.tile_id == current.tile_id { continue; }
        for per in p.permutation() {
            if current.edge(edge).eq(&per.opposite_edge(edge)) {

                let clean_parts = parts.into_iter()
                    .filter(|i| i.tile_id != p.tile_id && i.tile_id != current.tile_id)
                    .cloned()
                    .collect::<Vec<Part>>();

                return find_part_with_no_edge(edge, &per, &clean_parts);
            }
        }
    }

    return (*current).clone();
}


fn find_part_with_edge(edge: &Edge, current: &Part, parts: &Vec<Part>) -> Part {
    for p in parts {
        if p.tile_id == current.tile_id {
            continue;
        }

        for per in p.permutation() {
            if current.edge(edge).eq(&per.opposite_edge(edge)) {
                return per.clone();
            }
        }
    }

    unreachable!("Did not find a match for {} on edge {:?}", current.tile_id, edge);
}

fn scan_sea_monster_string(search: &String, map: &Vec<char>, offset: usize) -> Vec<usize> {
    let mut hits = Vec::new() as Vec<usize>;
    for i in 0..(map.len() - search.len()) {
        let split = map.split_at(i);
        let x = split.1.iter()
            .zip(search.chars())
            .all(|x| match x.1 {'#' => *x.0 == '#', _ => true });

        if x  && i >= offset { hits.push(i - offset); }
    }
    hits
}

fn solve_puzzle(mut data: &mut Vec<Part>, size: usize, top_left: Part, top_right: Part) -> Vec<Part> {
    let mut solve: Vec<Part> = Vec::with_capacity(144);

    let mut left = top_left;
    let mut right = top_right;

    (0..size).for_each(|y| {
        solve_line(&left, &right, &mut solve, &mut data, size);
        if y < size - 1 {
            left = find_part_with_edge(&Edge::Bottom, &left, &data);
            right = find_part_with_edge(&Edge::Bottom, &right, &data);
        }
    });
    solve
}

fn solve_line(left: &Part, right: &Part, solve: &mut Vec<Part>, data: &mut Vec<Part>, len: usize) {
    solve.push(left.clone());
    data.delete_part(&left);

    let mut search = (*left).clone();
    (1..len - 1).for_each(|_x| {
        let next: Part = find_part_with_edge(&Edge::Right, &search, &data);
        solve.push(next.clone());
        data.delete_part(&next);
        search = next;
    });

    solve.push(right.clone());
    data.delete_part(&right);
}

pub fn main() -> std::io::Result<()> {
    let mut data = read_lines_from_file("input.txt")?
        .split(|s| s.is_empty())
        .map(|a| a.to_vec())
        .map(Part::from)
        .collect::<Vec<Part>>();

    let num_part_sqrt_size = (data.len() as f32).sqrt() as usize;

    let no_left: Part = find_part_with_no_edge(&Edge::Left, &data.get(0).unwrap(), &data);
    let top_left: Part = find_part_with_no_edge(&Edge::Top, &no_left, &data);
    data.delete_part(&top_left);

    let top_right: Part = find_part_with_no_edge(&Edge::Right, &top_left, &data);
    data.delete_part(&top_right);

    let bottom_left: Part = find_part_with_no_edge(&Edge::Bottom, &top_left, &data);
    let bottom_right: Part = find_part_with_no_edge(&Edge::Right, &bottom_left, &data);

    println!("Result Part 1: {}", top_left.tile_id * top_right.tile_id * bottom_left.tile_id * bottom_right.tile_id);

    let solve = solve_puzzle(&mut data, num_part_sqrt_size, top_left, top_right);

    assert!(data.is_empty());

    let puzzle_edge_len = EDGE_LEN * num_part_sqrt_size;
    let sea_map_edge_len = (EDGE_LEN - 2) * num_part_sqrt_size;

    let mut sea_map: Vec<char> = Vec::new();

    for y in 0..puzzle_edge_len {
        if y % EDGE_LEN == 0 || y % EDGE_LEN == (EDGE_LEN - 1) {
            continue;
        }

        for x in 0..puzzle_edge_len {
            let part_index = (y / EDGE_LEN * num_part_sqrt_size) + x / EDGE_LEN;
            if x % EDGE_LEN == 0 || x % EDGE_LEN == (EDGE_LEN - 1) {
                continue;
            }
            sea_map.push(solve[part_index].get_at(x % EDGE_LEN, y % EDGE_LEN));
        }
    }

    for p in sea_map.permutation() {
        let hits1 = scan_sea_monster_string(&"                  # ".to_string(), &p, 0);
        let hits2 = scan_sea_monster_string(&"#    ##    ##    ###".to_string(), &p, (EDGE_LEN - 2) * num_part_sqrt_size * 1);
        let hits3 = scan_sea_monster_string(&" #  #  #  #  #  #   ".to_string(), &p, (EDGE_LEN - 2) * num_part_sqrt_size * 2);

        let set1= hits1.into_iter().collect::<HashSet<usize>>();
        let set2= hits2.into_iter().collect::<HashSet<usize>>();
        let set3= hits3.into_iter().collect::<HashSet<usize>>();

        let vec = set1.intersection(&set2).cloned().collect::<HashSet<usize>>().intersection(&set3).cloned().collect::<Vec<usize>>();
        if !vec.is_empty() {
            p.print(sea_map_edge_len);
            // println!("... there it is at {:?}", vec);
            let water = p.iter().filter(|c| **c == '#').count();
            // println!("Water count is {}", water);
            println!("Result Part 2: {}", water - (vec.len() * 15));
        }
    }
    Ok(())
}
