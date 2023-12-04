use std::fs;

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
                if total == 0 {
                    total = 1;
                } else {
                    total *= 2;
                }
            }
        }
    }

    total
}
fn main() {
    let contents = fs::read_to_string("./puzzle.txt").unwrap();
    let mut cards: Vec<Card> = Vec::new();
    for line in contents.lines() {
        cards.push(parse_card(line))
    }
    let mut total: i32 = 0;
    for card in cards {
        total += validate_card(card);
    }
    println!("Complete: total is {}", total);
}
