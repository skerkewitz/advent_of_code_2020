use std::collections::{HashMap, VecDeque};

mod test;

type T = i32;

fn push_turn_number(map: &mut HashMap<T, VecDeque<T>>, n: T, v: T) {

    if let Some(vec) = map.get_mut(&n) {
        vec.push_back(v);

        if vec.len() > 2 {
            vec.pop_front();
        }
        //     map.insert(n, vec.split_first().unwrap().1.to_vec());
        // }
        // } else {
        //     map.insert(n, vec.clone());
        // }
    } else {
        let mut vec1 = VecDeque ::with_capacity(3);
        vec1.push_back(v);
        map.insert(n, vec1);
    }
}

fn main() -> std::io::Result<()> {

    //let data = read_lines_from_file("sample.txt")?;

    let mut count_map = HashMap::new() as HashMap<T, T>;
    let mut turn_map = HashMap::new() as HashMap<T, VecDeque <T>>;

    let mut number = "0,6,1,7,2,19,20".split(',')
        .map(|s| s.parse::<T>().unwrap())
        .enumerate()
        .inspect(|i| {
            println!("Turn {} number {} (count 0)", i.0 + 1, i.1);
            count_map.insert(i.1, 1);
            push_turn_number(&mut turn_map, i.1, (i.0 + 1) as T);
            ()
        })
        .map(|i| i.1)
        .last().unwrap();

    println!();
    println!();

    let mut last_number = 20;
    count_map.insert(number, 0);
    for i in count_map.values().count() + 1..30000001 {
        let count = count_map.get(&last_number).unwrap_or(&0).clone();
        //println!("Turn {} last number {} (count {})", i, last_number, count);

        count_map.insert(last_number, count + 1);
        number = if count == 0 { 0 } else {
            // println!("Search last turn for {}", last_number);
            let last_turn = turn_map.get(&last_number).unwrap();
            let mut iter = last_turn.iter();
            let last2 = iter.next().unwrap();
            let last1 = iter.next().unwrap();
            // println!(" ... previous turn for {} was {} {} , new number is {}", last_number, last1, last2, last1 - last2);
            last1 - last2
        };


        last_number = number;
        push_turn_number(&mut turn_map, number, i as T);
    }

    println!("Result: {}", last_number);

    Ok(())
}
