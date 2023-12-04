use std::fs;
use std::collections::HashMap;

struct Round {
    blue: i32,
    green: i32,
    red: i32,
}

struct Game {
    id: i32,
    rounds: Vec<Round>,

}

fn get_max_numbers( rounds:Vec<Round>) -> HashMap<&'static str, i32> {
    let mut max_numbers:HashMap<&str, i32> = HashMap::new();
    max_numbers.insert("blue", 0);
    max_numbers.insert("green", 0);
    max_numbers.insert("red", 0);

    for round in rounds {
        if round.red > max_numbers["red"] {
            max_numbers.insert("red", round.red);
        }
        if round.green > max_numbers["green"] {
            max_numbers.insert("green", round.green);
        }
        if round.blue > max_numbers["blue"] {
            max_numbers.insert("blue", round.blue);
        }

        }
        return max_numbers
    }


   

 fn parse_round(round_string: &str) -> HashMap<&str, i32> {
    let mut numbers:HashMap<&str, i32> = HashMap::new();
    numbers.insert("blue", 0);
    numbers.insert("green", 0);
    numbers.insert("red", 0);
    let color_strings:Vec<&str> = round_string.split(',').collect();
    for color in color_strings {
        let data:Vec<&str> = color.trim().split(' ').collect();
        numbers.insert(data[1], data[0].parse::<i32>().unwrap());
    }
    return numbers;
 }

 fn parse_rounds(rounds_string:&str) -> Vec<Round>{
    let split_string:Vec<&str> = rounds_string.split(';').collect();
    let mut rounds:Vec<Round> = Vec::new();
    for round in split_string {
        let round_data = parse_round(round);
        let current_round = Round {blue: round_data["blue"], green: round_data["green"], red: round_data["red"]};
        rounds.push(current_round);
    }
    return rounds;
 }

fn validate_round(round:Round) -> bool {
    let max_red:i32 = 12;
    let max_blue:i32 = 14;
    let max_green:i32 = 13;

    if round.red > max_red {
        return false;
    } else if round.blue > max_blue {
        return false;
    } else if round.green > max_green {
        return false;
    }
    return true;
}

fn validate_game(game:Game) -> bool {

    for round in game.rounds {
        if validate_round(round) {
            ();
        } else {
            return false;
        }
    }
    return true;

}

fn main() {

    let mut sum = 0;

    let contents:String= fs::read_to_string("puzzle.txt").unwrap();
    let mut game_strings:Vec<&str> = Vec::new();
    let mut games:Vec<Game> = Vec::new();
    for line in contents.lines(){
        game_strings.push(line);
    }    

    for game in game_strings {
        let split_string:Vec<&str> = game.split(':').collect();
        let game_id:i32 = split_string[0].split(' ').collect::<Vec<&str>>()[1].parse().unwrap();
        let rounds_string:&str = split_string[1];
        let rounds = parse_rounds(rounds_string);
        let current_game = Game {
            id: game_id,
            rounds: rounds,
        };
        games.push(current_game);
    }

    for game in games {
        let id:i32 = game.id;
        let current_max_numbers = get_max_numbers(game.rounds);
        let power = current_max_numbers["red"] * current_max_numbers["blue"] * current_max_numbers["green"];
        sum += power;
    }
    println!("{}", sum);


}
