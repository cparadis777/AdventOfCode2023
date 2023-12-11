use std::cmp::Ordering;
use std::collections::HashMap;
use std::ops::Deref;
use std::{fmt, fs};

#[derive(Debug, Copy, Clone)]
enum HandType {
    None,
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    pub fn index(&self) -> usize {
        *self as usize
    }
}
impl fmt::Display for HandType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

#[derive(Clone)]
struct Hand {
    cards: Vec<i32>,
    bid: i32,
    hand_type: HandType,
}
impl Hand {
    pub fn cmp(&self, other: &Self) -> Ordering {
        // Check card order
        if self.hand_type.index() > other.hand_type.index() {
            return Ordering::Greater;
        } else if self.hand_type.index() < other.hand_type.index() {
            return Ordering::Less;
        }

        for (card1, card2) in self.cards.iter().zip(other.cards.iter()) {
            if card1 > card2 {
                return Ordering::Greater;
            } else if card1 < card2 {
                return Ordering::Less;
            }
        }
        return Ordering::Equal;
    }
    pub fn cmp_jokers(&self, other: &Self) -> Ordering {
        // Check card order
        if self.hand_type.index() > other.hand_type.index() {
            return Ordering::Greater;
        } else if self.hand_type.index() < other.hand_type.index() {
            return Ordering::Less;
        }

        for (card1, card2) in self.cards.iter().zip(other.cards.iter()) {
            if card1 > card2 {
                return Ordering::Greater;
            } else if card1 < card2 {
                return Ordering::Less;
            }
        }
        return Ordering::Equal;
    }
}

fn get_hand_type(cards: Vec<i32>) -> HandType {
    // Check if for N of a kind:
    let mut pairs_found: i32 = 0;
    let mut triples_found: i32 = 0;
    for card_value in 2..=14 {
        let n_occurences = cards.iter().filter(|x| **x == card_value).count();
        match n_occurences {
            5 => return HandType::FiveOfAKind,
            4 => return HandType::FourOfAKind,
            3 => triples_found += 1,
            2 => pairs_found += 1,
            _ => (),
        }
    }
    match (pairs_found, triples_found) {
        (1, 1) => HandType::FullHouse,
        (2, 0) => HandType::TwoPairs,
        (0, 1) => HandType::ThreeOfAKind,
        (1, 0) => HandType::OnePair,
        (_, _) => HandType::HighCard,
    }
}

fn get_hand_type_with_jokers(cards: Vec<i32>) -> HandType {
    // Check if for N of a kind:
    let mut pairs_found: i32 = 0;
    let mut triples_found: i32 = 0;
    let mut frequencies: HashMap<i32, i32> = HashMap::new();
    for value in 2..=24 {
        frequencies.insert(value, cards.iter().filter(|x| **x == value).count() as i32);
    }
    let joker_value = *frequencies.iter().max_by_key(|entry| entry.1).unwrap().0;
    let mut modified_cards: Vec<i32> = Vec::new();

    for card in cards.clone() {
        if card == 0 {
            modified_cards.push(joker_value);
        } else {
            modified_cards.push(card);
        }
    }

    println! {"{:?} modified to {:?}", cards, modified_cards};
    for card_value in 2..=14 {
        let n_occurences = modified_cards.iter().filter(|x| **x == card_value).count();
        match n_occurences {
            5 => return HandType::FiveOfAKind,
            4 => return HandType::FourOfAKind,
            3 => triples_found += 1,
            2 => pairs_found += 1,
            _ => (),
        }
    }
    match (pairs_found, triples_found) {
        (1, 1) => HandType::FullHouse,
        (2, 0) => HandType::TwoPairs,
        (0, 1) => HandType::ThreeOfAKind,
        (1, 0) => HandType::OnePair,
        (_, _) => HandType::HighCard,
    }
}

fn parse_part_one(puzzle: String) -> Vec<Hand> {
    let mut hands: Vec<Hand> = Vec::new();
    for line in puzzle.lines() {
        let mut split_line: Vec<&str> = line.split_whitespace().collect();
        let bid = split_line.pop().unwrap().parse::<i32>().unwrap();
        let mut cards: Vec<i32> = Vec::new();
        for card in split_line.pop().unwrap().chars() {
            if card.is_numeric() {
                cards.push(card.to_string().parse::<i32>().unwrap());
            } else {
                match card {
                    'A' => cards.push(14),
                    'K' => cards.push(13),
                    'Q' => cards.push(12),
                    'J' => cards.push(11),
                    'T' => cards.push(10),
                    _ => (),
                }
            }
        }
        let hand_type = get_hand_type(cards.clone());
        let mut new_hand: Hand = Hand {
            cards,
            bid,
            hand_type,
        };
        hands.push(new_hand);
    }

    hands
}

fn parse_part_two(puzzle: String) -> Vec<Hand> {
    let mut hands: Vec<Hand> = Vec::new();
    for line in puzzle.lines() {
        let mut split_line: Vec<&str> = line.split_whitespace().collect();
        let bid = split_line.pop().unwrap().parse::<i32>().unwrap();
        let mut cards: Vec<i32> = Vec::new();
        for card in split_line.pop().unwrap().chars() {
            if card.is_numeric() {
                cards.push(card.to_string().parse::<i32>().unwrap());
            } else {
                match card {
                    'K' => cards.push(13),
                    'Q' => cards.push(12),
                    'J' => cards.push(0),
                    'T' => cards.push(10),
                    'A' => cards.push(14),
                    _ => (),
                }
            }
        }
        let hand_type = get_hand_type_with_jokers(cards.clone());
        let mut new_hand: Hand = Hand {
            cards,
            bid,
            hand_type,
        };
        hands.push(new_hand);
    }

    hands
}
fn part_one() {
    let contents = fs::read_to_string("./puzzle.txt").unwrap();
    let mut hands = parse_part_one(contents);

    hands.sort_unstable_by(|a, b| a.cmp(b));
    //hands.reverse();
    let mut result: i32 = 0;
    for (index, hand) in hands.iter().enumerate() {
        let position: i32 = index as i32 + 1;
        result += hand.bid * position;
        //println!("{} * {} = {}", hand.bid, position, hand.bid * position);
    }
    println! {"{}", result};
    println!("Complete")
}

fn part_two() {
    let contents = fs::read_to_string("./puzzle.txt").unwrap();
    let mut hands = parse_part_two(contents);

    hands.sort_unstable_by(|a, b| a.cmp(b));
    //hands.reverse();
    let mut result: i32 = 0;
    for (index, hand) in hands.iter().enumerate() {
        let position: i32 = index as i32 + 1;
        result += hand.bid * position;
        //println!("{} * {} = {}", hand.bid, position, hand.bid * position);
    }
    println! {"{}", result};
    println!("Complete")
}

fn main() {
    //part_one();
    part_two();
}
