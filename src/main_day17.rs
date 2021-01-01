use std::collections::{HashMap, VecDeque, HashSet};
use std::fs::File;
use std::io::{BufReader, BufRead};
use regex::Regex;
use std::iter::FromIterator;
use std::cmp::{max, min};

mod test;

fn read_lines_from_file(filename: &str) -> std::io::Result<Vec<String>> {
    let f = File::open(filename)?;
    let reader = BufReader::new(f);
    return reader.lines().collect();
}

type I = i32;
type Vec4 = (I, I, I, I);
type Cube = HashMap<Vec4, bool>;

trait CubeHashMap {
    fn print_cube(&self);
    fn get_min_max(&self) -> ((I, I), (I, I), (I, I), (I, I));
    fn count_active_neighbors(&self, v: &Vec4) -> I;
    fn get_state(&self, v: &Vec4) -> bool;
    fn set_state(&mut self, v: &Vec4, state: bool);
    fn mutate(&self) -> Cube;
    fn count_current_actives(&self) -> I;
}

impl CubeHashMap for Cube {
    fn print_cube(&self) {
        let size = self.get_min_max();

        for w in size.3.0..=size.3.1 {
            for z in size.2.0..=size.2.1 {
                println!("z={}, w={}", z, w);
                for y in size.1.0..=size.1.1 {
                    for x in size.0.0..=size.0.1 {
                        let active = *self.get(&(x, y, z, w)).unwrap_or(&false);
                        if active { print!("#") } else { print!(".") };
                    }
                    println!()
                }
                println!()
            }
        }
    }

    fn get_min_max(&self) -> ((I, I), (I, I), (I, I), (I, I)) {
        let mut keys = self.keys();
        let s = keys.next().unwrap_or(&(0, 0, 0, 0));
        keys.fold(((s.0,s.0), (s.1,s.1), (s.2,s.2), (s.3,s.3)), |a, c| {
                (
                    (min(a.0.0, c.0), max(a.0.1, c.0)),
                    (min(a.1.0, c.1), max(a.1.1, c.1)),
                    (min(a.2.0, c.2), max(a.2.1, c.2)),
                    (min(a.3.0, c.3), max(a.3.1, c.3)),
                )
            })
    }

    fn count_active_neighbors(&self, v: &Vec4) -> I {
        let mut count = 0;
        for x in v.0 - 1..=v.0 + 1 {
            for y in v.1 - 1..=v.1 + 1 {
                for z in v.2 - 1..=v.2 + 1 {
                    for w in v.3 - 1..=v.3 + 1 {
                        if x == v.0 && y == v.1 && z == v.2 && w == v.3 {
                            continue;
                        }
                        if self.get_state(&(x, y, z, w)) {
                            count += 1
                        }
                    }
                }
            }
        }
        count
    }

    fn get_state(&self, v: &Vec4) -> bool {
        *self.get(v).unwrap_or(&false)
    }

    fn set_state(&mut self, v: &Vec4, state: bool) {
        if state { self.insert(*v, state); } else { self.remove(v); }
    }

    fn mutate(&self) -> Cube {
        let size = self.get_min_max();

        let mut mutated_cube: Cube = HashMap::new();
        for x in size.0.0 - 1..=size.0.1 + 1 {
            for y in size.1.0 - 1..=size.1.1 + 1 {
                for z in size.2.0 - 1..=size.2.1 + 1 {
                    for w in size.3.0 - 1..=size.3.1 + 1 {
                        let pos = (x, y, z, w);
                        let count = self.count_active_neighbors(&pos);

                        if self.get_state(&pos) {
                            mutated_cube.set_state(&pos, (count == 2 || count == 3));
                        } else {
                            mutated_cube.set_state(&pos, (count == 3));
                        }
                    }
                }
            }
        }

        mutated_cube
    }

    fn count_current_actives(&self) -> I {
        self.values().filter(|v| **v).count() as I
    }
}

fn main() -> std::io::Result<()> {

    fn map_active(chars: (usize, &String)) -> Vec<(Vec4, bool)> {
        chars.1.chars()
            .enumerate()
            .map(|c| ((c.0 as i32, chars.0 as i32, 0, 0), c.1 == '#'))
            .filter(|x| x.1)
            .collect()
    }

    let mut cube = read_lines_from_file("sample.txt")?.iter()
        .enumerate()
        .map( map_active)
        .flatten()
        .collect::<HashMap<Vec4, bool>>();

    for i in 1..=6 {
        cube = cube.mutate();
    }

    println!("Result: {}", cube.count_current_actives());
    Ok(())
}

