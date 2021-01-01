use std::collections::{HashMap, VecDeque, HashSet};
use std::fs::File;
use std::io::{BufReader, BufRead};
use regex::Regex;
use std::iter::{FromIterator, Peekable};
use std::cmp::{max, min};
use std::mem;

mod test;

fn read_lines_from_file(filename: &str) -> std::io::Result<Vec<String>> {
    let f = File::open(filename)?;
    let reader = BufReader::new(f);
    return reader.lines().collect();
}

type I = i64;

#[derive(Debug, Clone)]
enum LexerItem {
    Paren(char),
    Operand(char),
    Number(I),
}

impl LexerItem {

    fn lex(input: &String) -> Result<Vec<LexerItem>, String> {
        let mut result = Vec::new();

        let mut it = input.chars().peekable();
        while let Some(&c) = it.peek() {
            match c {
                '0'..='9' => {
                    it.next();
                    let n = LexerItem::get_number(c, &mut it);
                    result.push(LexerItem::Number(n));
                }
                '+' | '*' => {
                    result.push(LexerItem::Operand(c));
                    it.next();
                }
                '(' | ')' => {
                    result.push(LexerItem::Paren(c));
                    it.next();
                }
                ' ' => {
                    it.next();
                }
                _ => {
                    return Err(format!("unexpected character {}", c));
                }
            }
        }
        Ok(result)
    }

    fn get_number<T: Iterator<Item = char>>(c: char, iter: &mut Peekable<T>) -> I {
        let mut number = c.to_string().parse::<I>().expect("The caller should have passed a digit.");
        while let Some(Ok(digit)) = iter.peek().map(|c| c.to_string().parse::<I>()) {
            number = number * 10 + digit;
            iter.next();
        }
        number
    }
}

fn eval_number(lex: &LexerItem) -> I {
    match lex {
        LexerItem::Number(i) => *i,
        _ => panic!("Unexpected token {:?}", lex)
    }
}

fn eval_operand(lex: &LexerItem) -> char {
    match lex {
        LexerItem::Operand(i) => *i,
        _ => panic!("Unexpected token {:?}", lex)
    }
}

pub trait PeekableIterator : std::iter::Iterator {
    fn peek(&mut self) -> Option<&Self::Item>;
}

impl<I: std::iter::Iterator> PeekableIterator for std::iter::Peekable<I> {
    fn peek(&mut self) -> Option<&Self::Item> {
        std::iter::Peekable::peek(self)
    }
}

fn eval(prg: &mut PeekableIterator<Item=&LexerItem>) -> I {

    let mut acc = eval_number_or_term(prg);

    if let Some(n) = prg.peek() {
        if let LexerItem::Operand(_) = n {
        } else if let LexerItem::Paren(')') = n {
            return acc;
        } else {
            return acc;
        }
    }

    while let Some(lex1) = prg.peek() {

        if let LexerItem::Paren(')') = lex1 {

            return acc;
        }

       let lex2 = prg.next().unwrap();

        let op = eval_operand(lex2);

        let term : I;
        if op == '+' {
            term = eval_number_or_term(prg);
        } else {
            term = eval(prg);
        }

        print!("Calc: {} {} {} = ", acc, op, term);
        acc = match op {
            '+' => acc + term,
            '-' => acc - term,
            '*' => acc * term,
            '/' => acc / term,
            _ => panic!("Unknown operand {}", op)
        };
        println!("{}", acc);
    }

    return acc
}

fn eval_number_or_term(prg: &mut PeekableIterator<Item=&LexerItem>) -> I {
    let lex = prg.next().expect("Expect number");

    let acc: I;
    if let LexerItem::Paren('(') = lex {
        acc = eval(&mut prg.peekable());
        // prg.next().expect("End paren");
        println!("Term is: {}", acc);
    } else {
        acc = eval_number(&lex);
    }

    acc
}


fn main() -> std::io::Result<()> {

    let mut data = read_lines_from_file("sample.txt")?;

    let result = data.iter()
        .map(|s|{
            let prg = LexerItem::lex(s).unwrap();
            eval(&mut prg.iter().peekable())
        })
        .inspect(|i| println!("{}", i))
        .sum::<I>();

    println!("Result {}", result);
    Ok(())
}

