use std::fs::File;
use std::io::{BufRead, BufReader};

mod test;

fn read_lines_from_file(filename: &str) -> std::io::Result<Vec<String>> {
    let f = File::open(filename)?;
    let reader = BufReader::new(f);
    return reader.lines().collect();
}

type T = char;


const NORTH: T = 'N';
const SOUTH: T = 'S';
const EAST: T = 'E';
const WEST: T = 'W';
const LEFT: T = 'L';
const RIGHT: T = 'R';
const FORWARD: T = 'F';

fn turn_waypoint(waypoint: &mut (i32, i32), degrees: i32) {
    if degrees == 0 {
        return;
    }

    let sign = degrees.signum();
    let mut translated = *waypoint;
    if sign < 0 {
        translated.0 = if waypoint.1 > 0 { -waypoint.1.abs() } else { waypoint.1.abs() };
        translated.1 = if waypoint.0 > 0 { waypoint.0.abs() } else { -waypoint.0.abs() };
    } else {
        translated.0 = if waypoint.1 > 0 { waypoint.1.abs() } else { -waypoint.1.abs() };
        translated.1 = if waypoint.0 > 0 { -waypoint.0.abs() } else { waypoint.0.abs() };
    };

    waypoint.0 = translated.0;
    waypoint.1 = translated.1;
    return turn_waypoint(waypoint, degrees + (sign * 90 * -1));
}

fn go(pos: &mut (i32, i32), direction: char, amount: i32) {
    match direction {
        NORTH => pos.0 -= amount,
        SOUTH => pos.0 += amount,
        EAST => pos.1 += amount,
        WEST => pos.1 -= amount,
        _ => panic!("Invalid direction {}", direction)
    }
}

fn move_to_waypoint(pos: &mut (i32, i32), waypoint: &(i32, i32), amount: i32) {
    pos.0 += waypoint.0.clone() * amount;
    pos.1 += waypoint.1.clone() * amount;
}

fn step(i: (char, i32), pos: &mut (i32, i32), facing: char, waypoint: &mut (i32, i32)) -> char {
    match i.0 {
        NORTH | SOUTH | EAST | WEST => go(waypoint, i.0, i.1),
        LEFT => turn_waypoint(waypoint, i.1 * -1),
        RIGHT => turn_waypoint(waypoint, i.1),
        FORWARD => move_to_waypoint(pos, waypoint, i.1),
        _ => unreachable!("Unknown command {}", i.0)
    }

    return facing;
}


fn main() -> std::io::Result<()> {
    let data = read_lines_from_file("sample.txt")?.iter()
        .map(|s| s.split_at(1))
        .inspect(|d| println!("{} {}", d.0, d.1))
        .map(|d| (d.0.chars().next().unwrap(), d.1.parse::<i32>().unwrap() as i32))
        .inspect(|d| println!("{} {}", d.0, d.1))
        .collect::<Vec<(char, i32)>>();


    let mut facing = EAST;
    let mut pos = (0, 0);
    let mut waypoint = (-1, 10);

    for i in data {
        facing = step(i, &mut pos, facing, &mut waypoint);
        println!("After {} {} I'm at ({},{}) facing {} waypoint({},{})", i.0, i.1, pos.0, pos.1, facing, waypoint.0, waypoint.1)
    }



    println!("Result: {}", pos.0.abs() + pos.1.abs());

    Ok(())
}
