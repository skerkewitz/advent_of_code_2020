use std::collections::{HashMap, VecDeque, HashSet};
use std::fs::File;
use std::io::{BufReader, BufRead};
use regex::Regex;
use std::iter::FromIterator;
use std::ops::Try;

mod test;

type T = i32;

fn read_lines_from_file(filename: &str) -> std::io::Result<Vec<String>> {
    let f = File::open(filename)?;
    let reader = BufReader::new(f);
    return reader.lines().collect();
}

fn parse_values(re: &Regex, s: &str) -> Option<Vec<String>> {

    re.captures(s)?.iter()
        .skip(1)
        .map(|x| x.and_then(|x|Some(x.as_str().to_string())))
        .collect::<Option<Vec<String>>>()


}

type FieldType = (String, T, T, T, T);

fn fit_field(v: &i32, f: &FieldType) -> bool {
    (*v >= f.1 && *v <= f.2) || (*v >= f.3 && *v <= f.4)
}

fn possible_fields(fields: &Vec<FieldType>, v: &i32) -> Vec<String>{
    fields.iter()
        // .inspect(|a| println!("Be: {:?}", a))
        .filter(|f| fit_field(v, *f))
        // .inspect(|a| println!("Af: {:?}", a))
        .map(|f| f.0.to_string())
        .collect::<Vec<String>>()
}

fn field_name_for_row(values: &Vec<Vec<Vec<String>>>, all_fields: &HashSet<String>, i: usize) -> HashSet<String>{

    values.iter()
        .map(|v|v[i].iter().map(|x|x.to_owned()).collect::<HashSet<String>>())
        // .inspect(|a| println!("HasSet: {:?}", a))
        .fold(all_fields.clone(), |a, v|a.intersection(&v).map(|s|s.clone()).collect())
}

fn main() -> std::io::Result<()> {

    let data = read_lines_from_file("sample.txt")?
        .map(|a| a.ch)
        .split(|x| x.is_empty())
        // .inspect(|a| println!("D: {:?}", a))
        .map(|a| a.to_vec())
        .collect::<Vec<Vec<String>>>();

    println!("Debug: {:?}", data);

    let re = Regex::new(r"([a-z\W]+): ([0-9]+)-([0-9]+) or ([0-9]+)-([0-9]+)").unwrap();

    let fields = data.get(0).into_result()?.iter()
        .map(|s| parse_values(&re, s))
        .map(|v| (Some((v?[0].clone(), v?[1].parse(), v?[2].parse(), v?[3].parse(), v?[4].parse()))))
        .collect::<Option<Vec<(String, i32, i32,i32, i32)>>>();


    println!("Debug: {:?}", fields);


    let your_ticket = data.get(1).unwrap();
    let nearby_tickets = data.get(2).unwrap();


    let values = nearby_tickets.iter()
        .skip(1)
        .map(|s| s.split(",").map(|s|s.parse::<i32>().unwrap()).collect::<Vec<i32>>())
        // .inspect(|a| println!("-- Af: {:?}", a))
        .filter(|vc| vc.iter().all(|v|fields.iter().find(|f| fit_field(v, f)).is_some()))
        .map(|vc| vc.iter().map(|v| possible_fields(&fields, v)).inspect(|a|assert!(a.len()>0)).collect::<Vec<Vec<String>>>())
        .collect::<Vec<Vec<Vec<String>>>>();

    println!("Values: {:?}", values.first().unwrap());

    let mut all_field = fields.iter()
        .map(|i| i.0.clone())
        .collect::<HashSet<String>>();

    println!("All fields: {:?}", all_field);



    println!("----");
    println!("----");
    println!("----");
    println!("----");
    println!("----");
    println!("----");
    println!("----");

    let mut field_list = vec![] as Vec<Vec<String>>;

    for i in 0..fields.len() {

        let results = field_name_for_row(&values, &all_field, i);
        field_list.push(Vec::from_iter(results.iter().map(|x|x.clone())));
//        assert_eq!(results.len(), 1);

//        let f1 = results.iter().next().unwrap();
        println!("Field {}: {:?}", i, results);
  //      all_field.remove(f1);
//        field_list.push(f1.clone());
    }


    let mut clear_set = HashSet::new() as HashSet<String>;

    loop {

        let flat = field_list.iter()
            .find(|x|x.len() == 1 && !clear_set.contains(x.first().unwrap()));

        if flat == None {
            break;
        }

        let rf = flat.unwrap().first().unwrap();

        println!("Remove: {:?}", rf);
        clear_set.insert(rf.clone());

        field_list = remove_field(&field_list, rf);
        println!("Result: {:?}", field_list);
    }

    let your_ticket_number = your_ticket.iter().skip(1).next().unwrap().split(',').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    let final_field_list: i64 = field_list.iter()
        .enumerate()
        .map(|x| (x.0, x.1.first().unwrap().clone()))
        .filter(|x| x.1.starts_with("departure"))
        .inspect(|x|println!("{:?}", x))
        .map(|x| your_ticket_number[x.0] as i64)
        .product();


    // for i in 0..final_field_list.len() {
        println!("Field {}", final_field_list);
    // }



    //.last().unwrap();

    // println!();
    // println!();
    //
    // let mut last_number = 20;
    // count_map.insert(number, 0);
    // for i in count_map.values().count() + 1..30000001 {
    //     let count = count_map.get(&last_number).unwrap_or(&0).clone();
    //     //println!("Turn {} last number {} (count {})", i, last_number, count);
    //
    //     count_map.insert(last_number, count + 1);
    //     number = if count == 0 { 0 } else {
    //         // println!("Search last turn for {}", last_number);
    //         let last_turn = turn_map.get(&last_number).unwrap();
    //         let mut iter = last_turn.iter();
    //         let last2 = iter.next().unwrap();
    //         let last1 = iter.next().unwrap();
    //         // println!(" ... previous turn for {} was {} {} , new number is {}", last_number, last1, last2, last1 - last2);
    //         last1 - last2
    //     };
    //
    //
    //     last_number = number;
    //     push_turn_number(&mut turn_map, number, i as T);
    // }
    //
    // println!("Result: {}", last_number);

    Ok(())
}

fn remove_field(field_list: &Vec<Vec<String>>, flat: &String) -> Vec<Vec<String>> {
    field_list.iter()
        .map(|x| {
            if x.len() == 1 {
                x.clone()
            } else {
                x.iter()
                    .filter(|s| !(**s).eq(flat))
                    .map(|s| s.clone())
                    .collect()
            }
        })
        .collect()
}
