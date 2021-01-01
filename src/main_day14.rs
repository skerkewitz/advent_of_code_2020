use std::fs::File;
use std::io::{BufRead, BufReader};
use std::borrow::Borrow;

use regex::Regex;
use std::collections::HashMap;

mod test;

fn read_lines_from_file(filename: &str) -> std::io::Result<Vec<String>> {
    let f = File::open(filename)?;
    let reader = BufReader::new(f);
    return reader.lines().collect();
}

type T = usize;

fn build_mask(s: &str) -> (T, T) {

    let mask = s; //.split("=").skip(1).next().unwrap().trim();

    let mask_1 = mask.chars()
        .map(|c| {
            match c {
                'X' => '0',
                '1' => '1',
                '0' => '0',
                _ => panic!("Unknown char {}", c)
            }
        })
        .collect::<String>();

    let mask_0 = mask.chars()
        .map(|c| {
            match c {
                'X' => '1',
                '1' => '1',
                '0' => '0',
                _ => panic!("Unknown char {}", c)
            }
        })
        .collect::<String>();

    let m0 = usize::from_str_radix(mask_0.as_str(), 2).unwrap();
    let m1 = usize::from_str_radix(mask_1.as_str(), 2).unwrap();

    println!("Mask_0 {} {}", mask_0, m0);
    println!("Mask_1 {} {}", mask_1, m1);

    return (m0, m1)
}

fn to_str_mask(i: T) -> String {
    return format!("{:036b}", i)
}

fn parse_values(re: &Regex, s: &str) -> (T,T) {

    let v = re.captures(s).unwrap().iter()
        .skip(1)
        .map(|x| x.unwrap().as_str().to_string())
        .map(|x| (x.trim().parse::<T>().unwrap()))
        .collect::<Vec<T>>();

    return (v[0], v[1])
}


fn main() -> std::io::Result<()> {

    let re = Regex::new(r"[a-z]+\[([0-9]+)\] = ([0-9]+)").unwrap();
    let data = read_lines_from_file("sample.txt")?;

    let mut m = (0 as T,0 as T);
    let mut current_mak = "".to_string();

    let mut mem = HashMap::new() as HashMap<T, T>;
    for l in data {

        if l.starts_with("mask") {
            let mask = l.split("=").skip(1).next().unwrap().trim();
            current_mak = mask.clone().to_owned();
            let m_ = build_mask(mask);
            m.0 = m_.0;
            m.1 = m_.1;
        } else {
            let values = parse_values(&re, &l);

            let str_mask = to_str_mask(values.0);

            println!();
            println!("address: {} (decimal {})", str_mask, values.0);
            println!("mask     {}", current_mak);

            let result = str_mask.chars()
                .zip(current_mak.chars())
                .map(|x| match x.1 {
                    'X' => 'X',
                    '1' => '1',
                    '0' => x.0,
                    _ => panic!("Unknown char {}", x.1)
                })
                .collect::<String>();

            let mut possible_values = vec![] as Vec<String>;
            possible_values.push(result.clone());

            let mut did_change = true;
            while did_change {
                did_change = false;

                possible_values = possible_values.iter()
                    .map(|x| {
                        if x.contains('X') {
                            did_change = true;
                            vec![
                                x.replacen("X", "0", 1),
                                x.replacen("X", "1", 1)
                            ]
                        }
                        else {
                            vec![x.clone()]
                        }
                    })
                    .flatten()
                    .collect::<Vec<String>>();

            }

            for x in possible_values {
                let m0 = usize::from_str_radix(x.as_str(), 2).unwrap();
                println!("possible values  {} (decimal {})", x, m0);
                mem.insert(m0, values.1);
            }

            //println!("result   {}", result);
        }
    }

    println!("---");

    let result = mem.iter()
        .inspect(|d| println!("{} = {}", d.0, d.1))
        .map(|x| *x.1 as u64)
        .fold(0, |a, x| a + x) as u64;

    println!("Result: {}", result);

    Ok(())
}
