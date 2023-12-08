use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Error};
use counter::Counter;

pub fn day_seven_task_one(file_path: &str) -> Result<i32, Error> {
    let mut card_hands: Vec<CardHand> = Vec::new();
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let res_line = line.unwrap();
        let res = res_line.split(" ").collect::<Vec<&str>>();
        let hand = res[0];
        let weight = res[1].parse::<i32>().unwrap();
        let hand_type: HandType = get_hand_type(hand);
        card_hands.push(CardHand { cards: hand.to_owned(), hand_type, weight, rank: 1 })
    }
    for i in 0..card_hands.len() {
        for j in 0..card_hands.len() {
            if i != j {
                match card_hands[i].hand_type {
                    HandType::FiveOfAKind => {
                        if matches!(card_hands[j].hand_type, HandType::FiveOfAKind) {
                            if if_card_higher(&card_hands[i].cards, &card_hands[j].cards) {
                                card_hands[i].rank += 1;
                            }
                        } else {
                            card_hands[i].rank += 1;
                        }
                    },
                    HandType::FourOfAKind => {
                        if matches!(card_hands[j].hand_type, HandType::FourOfAKind) {
                            if if_card_higher(&card_hands[i].cards, &card_hands[j].cards) {
                                card_hands[i].rank += 1;
                            }
                        } else if !matches!(card_hands[j].hand_type, HandType::FiveOfAKind){
                            card_hands[i].rank += 1;
                        }

                    },
                    HandType::FullHouse => {
                        if matches!(card_hands[j].hand_type, HandType::FullHouse) {
                            if if_card_higher(&card_hands[i].cards, &card_hands[j].cards) {
                                card_hands[i].rank += 1;
                            }
                        } else if !matches!(card_hands[j].hand_type, HandType::FourOfAKind | HandType::FiveOfAKind){
                            card_hands[i].rank += 1;
                        }

                    },
                    HandType::ThreeOfAKind => {
                        if matches!(card_hands[j].hand_type, HandType::ThreeOfAKind) {
                            if if_card_higher(&card_hands[i].cards, &card_hands[j].cards) {
                                card_hands[i].rank += 1;
                            }
                        } else if matches!(card_hands[j].hand_type, HandType::HighCard | HandType::OnePair | HandType::TwoPair){
                            card_hands[i].rank += 1;
                        }      
                    },
                    HandType::TwoPair => {
                        if matches!(card_hands[j].hand_type, HandType::TwoPair) {
                            if if_card_higher(&card_hands[i].cards, &card_hands[j].cards) {
                                card_hands[i].rank += 1;
                            }
                        } else if matches!(card_hands[j].hand_type, HandType::HighCard | HandType::OnePair){
                            card_hands[i].rank += 1;
                        }
                    },
                    HandType::OnePair => {
                        if matches!(card_hands[j].hand_type, HandType::OnePair) {
                            if if_card_higher(&card_hands[i].cards, &card_hands[j].cards) {
                                card_hands[i].rank += 1;
                            }
                        } else if matches!(card_hands[j].hand_type, HandType::HighCard){
                            card_hands[i].rank += 1;
                        }
                    },
                    HandType::HighCard => {
                        if matches!(card_hands[j].hand_type, HandType::HighCard) && if_card_higher(&card_hands[i].cards, &card_hands[j].cards) {
                                card_hands[i].rank += 1;
                            }
                    },
                }
            }
        }
    }
    Ok(card_hands.iter().map(|x| x.rank*x.weight).sum())
}

pub fn day_seven_task_two(file_path: &str) -> Result<i32, Error> {
    let mut card_hands: Vec<CardHand> = Vec::new();
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let res_line = line.unwrap();
        let res = res_line.split(" ").collect::<Vec<&str>>();
        let hand = res[0];
        let weight = res[1].parse::<i32>().unwrap();
        let hand_type: HandType = get_hand_type_joker(hand);
        card_hands.push(CardHand { cards: hand.to_owned(), hand_type, weight, rank: 1 })
    }
    Ok(1)
}
#[derive(Debug)]
enum HandType {
    FullHouse,
    FiveOfAKind,
    FourOfAKind,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,

}
#[derive(Debug)]
struct CardHand {
    cards: String,
    hand_type: HandType,
    weight: i32,
    rank: i32
}

fn get_hand_type(hand: &str) -> HandType {
    let card_counter: Counter<char, i8> = hand.chars().collect();
    match card_counter.values().cloned().collect::<Vec<i8>>() {
        _x if _x.contains(&3) && _x.contains(&2) => HandType::FullHouse,
        _x if _x.contains(&5) => HandType::FiveOfAKind,
        _x if _x.contains(&4) => HandType::FourOfAKind,
        _x if _x.contains(&3) => HandType::ThreeOfAKind,
        _x if _x.iter().filter(|y| **y == 2).sum::<i8>() == 4 => HandType::TwoPair,
        _x if _x.iter().filter(|y| **y == 2).sum::<i8>() == 2 => HandType::OnePair,
        _ => HandType::HighCard,
    }
}

fn if_card_higher(card_1: &str, card_2: &str) -> bool {
    let char_vec = ['2','3','4','5','6','7','8','9','T', 'J', 'Q', 'K', 'A'];
    let card_values = char_vec.iter().enumerate().map(|(i, x)| (x, i+2)).collect::<HashMap<&char, usize>>();
    let card_char_1: Vec<char> = card_1.chars().collect();
    let card_char_2: Vec<char> = card_2.chars().collect();
    
    for i in 0..card_char_1.len() {
        if card_values.get(&card_char_1[i]) > card_values.get(&card_char_2[i]) {
            return true
        } else if card_values.get(&card_char_1[i]) == card_values.get(&card_char_2[i]) {
            continue
        }
        break
    }
    false
}

fn get_hand_type_joker(hand: &str) -> HandType {
    let card_counter: Counter<char, i8> = hand.chars().collect();
    if let Some(joker) = card_counter.get(&'J') {
        !todo!();
    } else {
        match card_counter.values().cloned().collect::<Vec<i8>>() {
        _x if _x.contains(&3) && _x.contains(&2) => HandType::FullHouse,
        _x if _x.contains(&5) => HandType::FiveOfAKind,
        _x if _x.contains(&4) => HandType::FourOfAKind,
        _x if _x.contains(&3) => HandType::ThreeOfAKind,
            _x if _x.iter().filter(|y| **y == 2).sum::<i8>() == 4 => HandType::TwoPair,
            _x if _x.iter().filter(|y| **y == 2).sum::<i8>() == 2 => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }
}

#[test]
fn test_day_seven_task_one() {
    let file_path = String::from("data/test-7.txt");
    let data = day_seven_task_one(&file_path).expect("err");

    assert_eq!(data, 243652)
}