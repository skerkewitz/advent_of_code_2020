use crate::utils;
use std::collections::HashSet;
use crate::main_day22::Player::{Player1, Player2};

type Card = usize;
type Score = usize;
type Deck = Vec<Card>;

#[derive(PartialEq)]
enum Player { Player1, Player2 }

fn calc_winner_score(deck: &Deck) -> Score {
    deck.iter().rev()
        .enumerate()
        .map(|x| (x.0 + 1) * (*x.1))
        .sum()
}

fn calc_winner_score_from_decks(d1: &Deck, d2: &Deck) -> (Player, Score) {
    if d1.is_empty() && !d2.is_empty() {
        (Player2, calc_winner_score(&d2))
    } else if !d1.is_empty() && d2.is_empty() {
        (Player1, calc_winner_score(&d1))
    } else {
        unreachable!("No empty deck, something is wrong");
    }
}

fn push_cards_to_deck(d1: &mut Deck, d2: &mut Deck, card_p1: Card, card_p2: Card) {
    if card_p1 > card_p2 {
        d1.push(card_p1);
        d1.push(card_p2);
    } else if card_p2 > card_p1 {
        d2.push(card_p2);
        d2.push(card_p1);
    } else {
        panic!("p1 == p2, something is wrong");
    }
}

fn recursive_combat(deck1: &mut Deck, deck2: &mut Deck) -> (Player, Score) {
    let mut d1 = deck1;
    let mut d2 = deck2;
    let mut played_hands: HashSet<(Deck, Deck)> = HashSet::new();

    while !d1.is_empty() && !d2.is_empty() {
        let hand = (d1.clone(), d2.clone());
        if played_hands.contains(&hand) {
            return (Player1, calc_winner_score(&d1));
        }
        played_hands.insert(hand);

        let card_p1 = d1.remove(0);
        let card_p2 = d2.remove(0);
        if card_p1 > d1.len() || card_p2 > d2.len() {
            push_cards_to_deck(&mut d1, &mut d2, card_p1, card_p2)
        } else {
            let result = recursive_combat(&mut d1[0..card_p1].to_vec(), &mut d2[0..card_p2].to_vec());
            if result.0 == Player1 {
                d1.push(card_p1);
                d1.push(card_p2);
            } else {
                d2.push(card_p2);
                d2.push(card_p1);
            }
        }
    }

    calc_winner_score_from_decks(&d1, &d2)
}

fn result_part1(deck1: &Deck, deck2: &Deck) -> Score {
    let mut d1 = deck1.clone();
    let mut d2 = deck2.clone();
    while !d1.is_empty() && !d2.is_empty() {
        let p1 = d1.remove(0);
        let p2 = d2.remove(0);
        push_cards_to_deck(&mut d1, &mut d2, p1, p2);
    }

    calc_winner_score_from_decks(&d1, &d2).1
}

fn result_part2(deck1: &Deck, deck2: &Deck) -> Score {
    recursive_combat(&mut deck1.clone(), &mut deck2.clone()).1
}

pub fn main() -> std::io::Result<()> {
    let data = utils::read_lines_from_file("inputs/day22/input.txt")?
        .split(|s| s.is_empty())
        .map(|a| a.to_vec())
        .collect::<Vec<Vec<String>>>();

    let deck_player1 = data[0].iter().skip(1).map(utils::string_to_number).collect::<Deck>();
    let deck_player2 = data[1].iter().skip(1).map(utils::string_to_number).collect::<Deck>();

    println!("Result day22 part1: {}", result_part1(&deck_player1, &deck_player2));
    println!("Result day22 part2: {}", result_part2(&deck_player1, &deck_player2));
    Ok(())
}


