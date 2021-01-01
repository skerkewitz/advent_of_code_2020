use std::collections::{HashMap, VecDeque, HashSet};
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::iter::{FromIterator, Peekable};
use std::cmp::{max, min};
use std::mem;
use std::env::join_paths;
use tokio::stream::StreamExt;
use regex::Regex;

mod test;

fn read_lines_from_file(filename: &str) -> std::io::Result<Vec<String>> {
    let f = File::open(filename)?;
    let reader = BufReader::new(f);
    return reader.lines().collect();
}

type I = usize;
type RuleIndex = I;
type Rule = String;
type Rules = HashMap<I, Rule>;
type RegexRule = Rule;
type RuleVault = HashMap<RuleIndex, RegexRule>;

fn solve_rule(rule_idx: RuleIndex, rules: &Rules, solved: &mut RuleVault) -> RegexRule {
    if let Some(s) = solved.get(&rule_idx) { return s.clone(); }

    let rule = rules.get(&rule_idx).expect("Rule");
    if rule.chars().all(|c| c.is_ascii_alphabetic()) {
        solved.insert(rule_idx, rule.clone());
        return rule.clone();
    }

    let converted_rules = rule.split('|')
        .map(|s| {
            s.split(' ')
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<RuleIndex>().expect("Rule index"))
                .collect::<Vec<I>>()
        })
        .collect::<Vec<Vec<I>>>();

    let mut simple_rules = vec![];
    let mut recursive_rules = vec![];
    for r in converted_rules {
        if r.contains(&rule_idx) { recursive_rules.push(r); } else { simple_rules.push(r); }
    }

    let rx_simple = simple_rules.iter()
        .map(|v| {
            v.iter().map(|ri|  solve_rule(*ri, rules, solved)).collect::<Vec<RegexRule>>().join("")
        })
        .collect::<Vec<RegexRule>>()
        .join("|");

    let rx_recursive = recursive_rules.iter()
        .map(|v| {
            v.iter()
                .map(|ri| if *ri == rule_idx { "#".to_string() } else { solve_rule(*ri, rules, solved) })
                .collect::<Vec<RegexRule>>()
                .join("")
        })
        .map(|s| {
            let d = s.split("#")
                .map(|s| s.to_string())
                .collect::<Vec<RegexRule>>();
            format!("(?P<x{}>({})(?P>x{})?({}))", rule_idx, d.get(0).unwrap(), rule_idx, d.get(1).unwrap()).to_string()
        })
        .inspect(|s| println!("Recursive ruless {}", s))
        .collect::<Vec<RegexRule>>()
        .join(" | ");

    let flatten = vec![rx_simple, rx_recursive].into_iter()
        .filter(|s| !s.is_empty())
        .collect::<Vec<Rule>>()
        .join("|");

    let final_rule = format!("({})", flatten);
    solved.insert(rule_idx, final_rule.clone());
    return final_rule;
}

fn main() -> std::io::Result<()> {
    let data = read_lines_from_file("input.txt")?
        .split(|s| s.is_empty())
        .map(|a| a.to_vec())
        .collect::<Vec<Vec<String>>>();

    fn map_rule(s: &String) -> (usize, String) {
        let split = s.split(':').collect::<Vec<&str>>();
        (split[0].parse::<I>().expect("Index"), split[1].replace("\"", "").trim().to_string())
    }

    let rules = data.get(0).expect("Rules").iter().map(map_rule).collect::<Rules>();

    let mut vault: RuleVault = HashMap::new();
    for x in rules.iter().enumerate() {
        let result = solve_rule(*x.1.0, &rules, &mut vault);
        println!("Result {:02} {:?} = {}", x.1.0, x.1.1, result);
    }

    let regex_rule = vault.get(&0).expect("RegexRule");
    let regex = Regex::new(format!("^{}$", regex_rule).as_str()).unwrap();

    let messages = data.get(1).unwrap().clone();
    let result = messages.iter()
        .filter(|x| regex.is_match(x.as_bytes()).expect("Match or not"))
        .collect::<Vec<&String>>()
        .len();

    println!("Result {}", result);
    Ok(())
}



