use std::fs;
use std::collections::HashMap;


#[derive(Clone)]
struct Card {
    id: i32,
    pool: Vec<i32>,
    winning: Vec<i32>,
}

fn parse_card(s: &str) -> Card {
    let mut new_card = Card {
        id: 0,
        pool: Vec::new(),
        winning: Vec::new(),
    };
    let first_parse: Vec<&str> = s.split(':').collect();
    let id = first_parse[0]
        .split(' ')
        .collect::<Vec<&str>>()
        .pop()
        .unwrap()
        .parse::<i32>()
        .unwrap();

    let second_parse: Vec<&str> = first_parse[1].split('|').collect();
    let pool_string = second_parse[0].trim();
    let winning_string = second_parse[1].trim();

    let mut pool_vec: Vec<i32> = Vec::new();
    let mut winning_vec: Vec<i32> = Vec::new();

    for number in pool_string.split(' ') {
        let parsed_number = number.trim().parse::<i32>();
        match parsed_number {
            Ok(number) => pool_vec.push(number),
            Err(_err) => (),
        }
    }
    for number in winning_string.split(' ') {
        let parsed_number = number.trim().parse::<i32>();
        match parsed_number {
            Ok(number) => winning_vec.push(number),
            Err(_err) => (),
        }
    }
    new_card.id = id;
    new_card.pool = pool_vec.clone();
    new_card.winning = winning_vec.clone();
    return new_card;
}

fn validate_card(card: Card) -> i32 {
    let mut total = 0;
    let winning = card.winning.clone();
    for test_number in card.pool {
        for winning_number in &winning {
            if test_number == *winning_number {
                total += 1;
            }
        }
    }

    total
}

fn part_one() {

    let contents = fs::read_to_string("./puzzle.txt").unwrap();
    let mut cards: Vec<Card> = Vec::new();
    for line in contents.lines() {
        cards.push(parse_card(line));
    }
    let mut total: i32 = 0;
    for card in cards {
        let number_of_match:i32 = validate_card(card);
        if number_of_match != 0 {
            total += 1 * i32::pow(2, (number_of_match-1).try_into().unwrap());
        }
    }
    println!("Complete: total is {}", total);

}

fn part_two() {
    let contents = fs::read_to_string("./puzzle.txt").unwrap();
    let mut cards: Vec<Card> = Vec::new();
    for line in contents.lines() {
        cards.push(parse_card(line))
    }

    let mut collection:HashMap<i32, i32> = HashMap::new();

    for card in cards.clone() {
        collection.insert(card.id, 1);
    }

    for card in cards {
        let card_id = card.id;
        let number_of_matches:i32 = validate_card(card);
        println!("card #{}: {} matches", card_id, number_of_matches);
        let current_number_of_cards = collection[&card_id];
        for offset in 1..=number_of_matches{
            let card_to_add_id = card_id+offset;
            let current_number_of_card_to_add = collection[&(card_to_add_id)];
            let new_number_of_cards = current_number_of_cards + current_number_of_card_to_add;
            collection.insert(card_to_add_id, new_number_of_cards);

        }
        
    }
    let mut total = 0;
    for number_of_cards in collection.values() {
        total += number_of_cards;

    }
    println!("{:?}", collection);
    println!("{}", total);
}

fn main() {
    //part_one();
    part_two();
    
}