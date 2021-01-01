use std::fs::File;
use std::io::{BufRead, BufReader};
use std::borrow::Borrow;

mod test;

fn read_lines_from_file(filename: &str) -> std::io::Result<Vec<String>> {
    let f = File::open(filename)?;
    let reader = BufReader::new(f);
    return reader.lines().collect();
}

type T = char;

fn is_multiple(check: i128, base: i128) -> bool {
    return check % base == 0;
}

fn chinese_remainder(n: &[i128], a: &[i128]) -> i128 {
    let prod: i128 = n.iter().product();

    let mut p = 0;
    let mut sm = 0;

    for i in 0..n.len() {
        p = prod / n[i];
        sm += a[i] * mulInv(p, n[i]) * p;
    }

    return sm % prod;
}

fn mulInv(a: i128, b: i128) -> i128 {

    let mut a_ = a;
    let mut b_ = b;

    let b0 = b;
    let mut x0 = 0;
    let mut x1 = 1;

    if b == 1 {
        return 1;
    }

    while a_ > 1 {
        let q = a_ / b_;
        let amb = a_ % b_;
        a_ = b_;
        b_ = amb;
        let xqx = x1 - q * x0;
        x1 = x0;
        x0 = xqx;
    }

    if x1 < 0 {
        x1 += b0;
    }

    return x1;
}


fn main() -> std::io::Result<()> {
    let earliest_time = 1008833;
    let input = "19,x,x,x,x,x,x,x,x,41,x,x,x,x,x,x,x,x,x,643,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,17,13,x,x,x,x,23,x,x,x,x,x,x,x,509,x,x,x,x,x,37,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,29".to_string();

    let mut data = input.split(',')
        .enumerate()
        .filter(|s| !s.1.eq("x"))
        .map(|s| (s.0 as i128, s.1.parse::<i128>().unwrap()))
        .inspect(|d| println!("{} {}", d.0, d.1))
        .collect::<Vec<(i128, i128)>>();


    // let max = data.iter().max_by(|x, y| x.1.cmp(&y.1)).unwrap();
    // let inc = max.1;
    //let inc = data.first().unwrap().1;
    // println!("Max is: {} {}", max.0, max.1);


    // let n = [17, 13, 19];
    // let a = [0, 11, 16];
    // println!("Chinese: {}", chinese_remainder(&n, &a));

    let n1 = data.iter().map(|x| x.1.clone()).collect::<Vec<i128>>();
    let a1 = data.iter().map(|x| (x.1-(x.0 % x.1)).clone()).collect::<Vec<i128>>();
    println!("Chinese: {}", chinese_remainder(n1.as_slice(), a1.as_slice()));

    // let mut t = inc;
    // // let mut c = 0;
    // while true {
    //     let tt = t - max.0;
    //
    //     // c += 1;
    //     if data.iter()
    //         //.skip(1)
    //         .all(|x| is_multiple(tt + x.0, x.1)) {
    //         println!("Result: {}", tt);
    //         break;
    //     }
    //
    //     if t % 100000000 == 0 {
    //         println!("... thinking: {}", t);
    //     }
    //
    //     t += inc;
    // }

    // println!("Checks: {}", c);
    //
    // let mut facing = EAST;
    // let mut pos = (0, 0);
    // let mut waypoint = (-1, 10);
    //
    // for i in data {
    //     facing = step(i, &mut pos, facing, &mut waypoint);
    //     println!("After {} {} I'm at ({},{}) facing {} waypoint({},{})", i.0, i.1, pos.0, pos.1, facing, waypoint.0, waypoint.1)
    // }

    // println!("Result: {}", pos.0.abs() + pos.1.abs());

    Ok(())
}
