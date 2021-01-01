mod test;

fn parse_amount(amount_str: &str) -> (i32, i32) {
    assert!(!amount_str.is_empty());
    let split: Vec<&str> = amount_str.split('-').collect();

    assert_eq!(split.len(), 2);

    let min = split[0];
    let max = split[1];

    let m0 = min.parse::<i32>().unwrap();
    let m1 = max.parse::<i32>().unwrap();

    return (m0, m1);
}


fn is_valid(input: String) -> bool {
    assert!(!input.is_empty());

    let split: Vec<&str> = input.split(' ').collect();

    /* We expect three parts. */
    assert_eq!(split.len(), 3);

    let amount = parse_amount(split[0]);
    let char = split[1].strip_suffix(':').unwrap();
    let password = split[2];

    println!("Check {}-{}, {} and {}", amount.0, amount.1, char, password);


    let x = char.chars().next().unwrap();

    let mut pos = 0;
    let mut has_match = false;
    for c in password.chars() {
        pos += 1;

        if c == x && (pos == amount.0 || pos == amount.1) {
            if has_match {
                return false;
            }
            has_match = true;
        }
    }

    return has_match;
}

async fn download_data(target: &str) {
    //let target = "https://www.rust-lang.org/logos/rust-logo-512x512.png";
    match reqwest::get(target).await {
        Err(why) => panic!("Could not get response {:?}", why),
        Ok(response) => match response.text().await {
            Err(why) => panic!("Could not get text {:?}", why),
            Ok(content) => println!("Content {}", content)
        },
    }
}

use std::fs::File;
use std::io::{BufReader, BufRead};
use std::fmt;
use std::collections::HashSet;
use tokio::time::delay_queue::Key;

fn read_lines_from_file(filename: &str) -> std::io::Result<Vec<String>> {
    let mut f = File::open(filename)?;
    let mut reader = BufReader::new(f);

    let mut data: Vec<String> = vec!();
    for s in reader.lines() {
        data.push(s?)
    }

    return Ok(data);
}

fn char_at(data: &Vec<u8>, x: i32, y: i32, stride: i32) -> char {
    return data[(y * (stride + 1) + x % stride) as usize] as char;
}


//use tokio::runtime::Builder;
// #[tokio::main]

fn count_trees(data: &Vec<u8>, slopex: i32, slopey: i32) -> u64 {
    let mut x = 0 as i32;
    let mut y = 0 as i32;
    let mut tree = 0 as i32;

    let stride = data.iter().position(|&it| it as char == '\n').unwrap() as i32;

    let rows = data.iter().filter(|&it| *it as char == '\n').count() as i32;
    println!("stride is {} rows is {}", stride, rows);

    while y < rows - 1 {
        x += slopex;
        y += slopey;

        let c = char_at(&data, x, y, stride);

        if c == '#' {
            tree += 1;
            // println!("Tree at {}, {}", x, y + 1);
        }
    }

    println!("Tree at {} ", tree);
    return tree as u64;
}

struct KeyValue {
    key: String,
    value: String,
}
use lazy_static::lazy_static;

lazy_static! {
    static ref eye_colors: HashSet<String> = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
        .iter()
        .map(|s| String::from(*s))
        .collect();
}

impl KeyValue {

    fn parse_from_string(s: &str) -> KeyValue {
        assert!(!s.is_empty());

        let pair = s.split(":")
            // .inspect(|x| println!("Before map: {}", x))
            .collect::<Vec<&str>>();

        assert_eq!(pair.len(), 2);

        let k = String::from(pair[0]);
        let v = String::from(pair[1]);

        KeyValue { key: k, value: v }
    }

    fn is_valid(&self) -> bool {
        match self.key.as_str() {
            "byr" => return KeyValue::range_check(&self.value, 1920, 2002),
            "iyr" => return KeyValue::range_check(&self.value, 2010, 2020),
            "eyr" => return KeyValue::range_check(&self.value, 2020, 2030),
            "hgt" => return KeyValue::height_check(&self.value),
            "hcl" => return KeyValue::hair_color_check(&self.value),
            "ecl" => return eye_colors.contains(self.value.as_str()),
            "pid" => return self.value.len() == 9 && self.value.chars().all(char::is_numeric),
            "cid" => return true,
            _ => panic!("Its something else {}", self.key)
        }
        return true;
    }

    fn hair_color_check(v: &String) -> bool {

        if v.chars().next().unwrap() != '#' {
            return false
        }

        let values = v.split_at(1);

        if values.1.len() != 6 {
            return false
        }

        return values.1.chars().all(|c| c.is_ascii_hexdigit());
    }

    fn height_check(v: &String) -> bool {
        let values = v.split_at(v.len() - 2);

        return match values.1 {
            "cm" => KeyValue::range_check(&values.0.to_owned(), 150, 193),
            "in" => KeyValue::range_check(&values.0.to_owned(), 59, 76),
            _ => false
        }
    }

    fn range_check(v: &String, low: i32, high: i32) -> bool {
        return match v.parse::<i32>() {
            Err(e) => false,
            Ok(value) => value >= low && value <= high
        }
    }
}

// To use the `{}` marker, the trait `fmt::Display` must be implemented
// manually for the type.
impl fmt::Display for KeyValue {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "Key:{}, Value:{}", self.key, self.value)
    }
}

fn parse_keys(s: &String) -> Vec<KeyValue> {
    let mut vec = s.split(" ")
        .map(|e| KeyValue::parse_from_string(e))
        .filter(|e| e.is_valid())
        .collect::<Vec<KeyValue>>();
    vec.sort_by(|a, b| a.key.cmp((&b.key)));
    return vec;
}

fn main() -> std::io::Result<()> {
    let ref_set: HashSet<String> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid" /*, "cid"*/]
        .iter()
        .map(|s| String::from(*s))
        .collect();


    let lines = read_lines_from_file("input.txt")?
        .split(|s| s.is_empty())
        .map(|a| a.join(" "))
        // .inspect(|x| println!("One: {}", x))
        .map(|a| parse_keys(&a).iter().map(|a| a.key.clone()).collect::<HashSet<String>>())
        .map(|a| a.intersection(&ref_set).count() == 7)
        .filter(|a| *a)
        .count();
    // .collect::<Vec<bool>>();

    println!("Result is: {}", lines);
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
