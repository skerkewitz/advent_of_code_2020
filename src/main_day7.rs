mod test;

use std::fs::File;
use std::io::{BufReader, BufRead};
use std::fmt;
use std::collections::{HashSet, HashMap};
use tokio::time::delay_queue::Key;
use std::iter::FromIterator;
use tokio::stream::StreamExt;
use std::collections::hash_map::Iter;

fn read_lines_from_file(filename: &str) -> std::io::Result<Vec<String>> {
    let mut f = File::open(filename)?;
    let mut reader = BufReader::new(f);
    return reader.lines().collect();
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


fn count_answers(data: &[String]) -> usize {
    return match data.len() {
        0 => 0,
        1 => data[0].len(),
        _ => data.iter()
            .skip(1)
            .map(|s| s.chars().collect::<HashSet<char>>())
            .fold(data[0].chars().collect::<HashSet<char>>(), |a, hs| a.intersection(&hs).map(|c| *c).collect())
            .len()
    };
}

struct BagValue {
    key: String,
    value: Vec<(i32, String)>,
}

impl BagValue {
    fn from(str: &String) -> BagValue {
        assert!(!str.is_empty());

        let split = str.split("contain").collect::<Vec<&str>>();

        let bags = split[1].split(",")
            .filter(|s| " no other bags.".ne(*s))
            .map(|s| s.trim().split(' ').take(3).collect::<Vec<&str>>().join(" "))
            .collect::<Vec<String>>();

        let v = bags.iter()
            .map(|s| s.split_at(s.find(' ').unwrap()))
            .map(|s| (s.0.parse::<i32>().unwrap() as i32, s.1.trim().to_string()))
            .inspect(|s| println!("... {} - {}", s.0, s.1))
            .collect::<Vec<(i32, String)>>();

        let string = split[0].replace(" bags", "").trim().to_string();
        return BagValue { key: string, value: v };
    }

    fn count_bag(&self, search: &str) -> i32 {
        return if self.key.eq(search) { 1 } else { 0 };
    }

    fn count_sub_bags(&self, search: &str) -> i32 {
        return self.value.iter()
            .filter(|s| search.eq(&s.1))
            .fold(0, |a, x| a + &x.0);
    }

    fn count_bags_inside(&self, data: &Vec<BagValue>, search: &str, map:&mut HashMap<String, i32>) -> i32 {

        println!("Count inside -> {}, search -> {}", self.key, search);
        if self.value.len() == 0 {
            println!("No sub bags in {}", self.key);
            return 1;
        }

        return self.value.iter()
      //      .inspect(|x| println!(".....sub check {}", x.1))
            .map(|x| {
               //  if x.1.eq(search) {
               // //     println!("Leaf node");
               //      return 1;
               //  }


                let inside = match map.get(&x.1) {
                    None => {
                        let n = data.iter()
                            .find(|y| y.key.eq(&x.1))
                            .unwrap()
                            .count_bags_inside(data, search, map);

                        println!("Cache store {} {}", x.1, n);
                        map.insert(x.1.clone(), n);
                        n
                    },
                    Some(vn) => *vn
                };

                println!("Sum {} is {} {}", x.1, x.0, inside);
                let t = (inside * &x.0, &x.1);
                return t;
            })
            .inspect(|x| println!("...... sub {} is {}", x.0, x.1))
            .fold(1, |a, x| a + x.0);
    }
}

fn count(data: &Vec<BagValue>, search: &str) -> i32 {
    eprintln!("Search {}", search);

    // let hit = data.iter()
    //     .find(|x| x.key.eq(search)).unwrap();

    return data.iter()
        .filter(|x| x.key.eq(search))
        .inspect(|x| println!("Root {}", x.key))
        .map(|x| (&x.key, x.count_bags_inside(data, search, &mut HashMap::new())))

        .inspect(|x| println!("Result {} = {}\n", x.0, x.1))
        // .map(|x| x.1 > 0)
        // .filter(|x|*x == true)
        .fold(0, |a, x| a + x.1);
        //.count() as i32;
}

fn main() -> std::io::Result<()> {

    let mut x = String::from("hello");
    let a = &mut x;
    let b = &mut x;

    println!("{}\n", a);


    let search = "shiny gold";
    //let search = "dark green";

    let bags = read_lines_from_file("input.txt")?.iter()
        .inspect(|s| println!("org: {}", s))
        //.split(|s| s.is_empty())
        .map(|x| BagValue::from(x))
        .collect::<Vec<BagValue>>();
//         .map(|x| (x.count_bag(search), x.count_sub_bags(search)))
    // .inspect(|s| println!("B: {}, {}", s.0, s.1))
    // .fold(0, |a, x| return a + x.0 + x.1);


    println!("\n");
    println!("\n");

    let lines = count(&bags, search);
    println!("Result is: {}", lines);

    Ok(())
}
