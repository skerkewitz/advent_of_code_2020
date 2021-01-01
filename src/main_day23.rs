#![feature(map_first_last)]
use rayon::prelude::*;
use crate::utils;
use std::collections::BTreeSet;
use std::collections::{LinkedList, VecDeque};
use std::iter::FromIterator;

use std::time::SystemTime;
use std::cmp::{max, min};

use std::thread;

type Cup = u32;
// type Cups = LinkedList<Cup>;
type Cups = Vec<Cup>;
//type Cups = VecDeque<Cup>;

#[inline(always)]
fn index_of_cup(cups: &mut dyn Iterator<Item=&Cup>, search: Cup) -> Option<usize> {
    for (idx, i) in cups.enumerate() {
        if *i == search {
            return Some(idx);
        }
    }

    None
}

// laufe einmal durch das set.
// wenn du current findest dann return position
// merke die position von max
// merke dir position von größter zahl kleiner current größer 0



#[inline(always)]
fn find_dest_index(remaining: &[Cup], current: Cup) -> usize {

    // let length = remaining.len();
    // let chunk_size = if length < 1000 { length } else { length / 16 };
    //
    // let data_sets = remaining.chunks(chunk_size).enumerate()
    //     .map(|(offset, data)| (offset * chunk_size, data))
    //     .collect::<Vec<_>>();

    let results = remaining.par_iter().enumerate()
        .find_any(|(offset, data)| { 
//            let handle = thread::current();
            //println!("{:?}", handle.id());
            **data == current
        });
        // .inspect(|x|println!("{:?}", x))
        //.collect::<Vec<()>>();

    results.unwrap().0
}



#[inline(always)]
fn round_fast(cups: &mut Cups) {

    //println!("Cups: {:?}", &cups);

    let current = *cups.first().unwrap();
    let p1 = cups[1];
    let p2 = cups[2];
    let p3 = cups[3];

    //let mut pick_up: Vec<_> =  Vec::from(&cups[1..4]);

    let len = *&cups.len();
//    cups.copy_within(4..len, 0);

    let mut new_current = current -1;
    while new_current == p1 || new_current == p2 || new_current == p3 || new_current == 0 {
        if new_current == 0 {
            new_current = cups.len() as u32;
        } else if (new_current == current) {
            panic!("Something is wrong")
        } else {
          new_current -= 1;
        }
    }

//    println!("Current     : {}", current);
    //println!("New current : {}", new_current);

    let dest_index = find_dest_index(&mut cups[4..(len)], new_current);

    // println!("Search took: {}", now.elapsed().unwrap().as_micros());

    //let mut copy = Vec::from(cups.as_slice());
    cups.copy_within(4..dest_index+4+1, 0);  // new head
    // println!("Cups acopy : {:?}", &copy);
    cups[dest_index+1] = p1;
    cups[dest_index+2] = p2;
    cups[dest_index+3] = p3;
    // println!("Copy alen  : {:?}", &copy);
    cups.copy_within((dest_index+4+1)..len, dest_index+4);  // new head
    cups[len-1] = current;

    // println!("Copy took {}", now.elapsed().unwrap().as_micros());
}

fn result_part1(data: &Cups) -> String {
    let mut cups = &mut data.clone();
    for moves in 1..=100 {
        println!("\n-- move {} --", moves);
        round_fast(&mut cups);
    }

    println!("-- final --");
    println!("cups: {:?}", &cups);

    let idx = index_of_cup(&mut cups.iter(), 1).unwrap();
    let remaining_after = cups.split_off(idx);
    let remaining_before = cups.to_owned();
    let mut result = Vec::with_capacity(data.len());
    result.extend(remaining_after);
    result.extend(remaining_before);

    result.iter().filter(|i| **i != 1).map(|i| format!("{}", i)).collect::<Vec<String>>().join("")
}

fn result_part2(data: &Cups) -> usize {
    let mut cups = &mut data.clone();

    for n in 10..=1000000 {
        cups.push(n);
    }

    for moves in 1..=10000000 {
    //for moves in 1..=10000 {
        if moves % 50000 == 0 {
            println!("\n-- move {} --", moves);
        }
        //let now = SystemTime::now();
        round_fast(&mut cups);
        //println!("Search took: {}", now.elapsed().unwrap().as_micros());
        // match now.elapsed() {
        //     Ok(elapsed) => {
        //         // it prints '2'
        //         println!("Took: round {}", elapsed.as_micros());
        //     }
        //     Err(e) => {
        //         // an error occurred!
        //         println!("Error: {:?}", e);
        //     }
        // }
    }

    //println!("-- final --");
    //println!("cups: {:?}", cups);

    let idx = index_of_cup(&mut cups.iter(), 1).unwrap();
    let remaining_after = cups.split_off(idx);
    let remaining_before = cups.to_owned();
    let mut result = Vec::with_capacity(data.len());
    result.extend(remaining_after);
    result.extend(remaining_before);

    result[0] as usize * result[1] as usize
}


pub fn main() -> std::io::Result<()> {
    rayon::ThreadPoolBuilder::new().num_threads(64).build_global().unwrap();

    //let input = "389125467".to_string(); // sample
    let input ="716892543".to_string();

    let data = input.chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>();

    println!("{:?}", data);


    //let mut list = LinkedList::from_iter(data.iter().cloned());

    //utils::assert_and_print_result("day23, part1", "49725386".to_string(), result_part1(&data));
    println!("Result part2: {:?}", result_part2(&data));

    
    //utils::assert_and_print_result("day01, part2", 232508760, result_part2(&data));
    Ok(())
}


