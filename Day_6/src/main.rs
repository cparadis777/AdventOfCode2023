use std::fs;

#[derive(Clone, Copy)]
struct Race {
    time: i128,
    distance: i128,
}

fn parse_line(line: &str) -> Vec<i128> {
    line.split(' ')
        .filter(|x| !x.is_empty())
        .filter(|x| x.find(':').is_none())
        .map(|x| x.parse::<i128>().unwrap())
        .collect()
}

fn parse_line_part_two(line: &str) -> i128 {
    line.split(' ')
        .filter(|x| !x.is_empty())
        .filter(|x| x.find(':').is_none())
        .collect::<Vec<&str>>()
        .join("")
        .parse::<i128>()
        .unwrap()
}

fn compute_traveled_distance(total_time: i128, charge_time: i128) -> i128 {
    let travel_time = total_time - charge_time;
    let speed: i128 = charge_time;
    speed * travel_time
}

fn solve_race(race: Race) -> i128 {
    let mut number_of_wins: i128 = 0;
    for charge_time in 0..=race.time {
        if compute_traveled_distance(race.time, charge_time) > race.distance {
            number_of_wins += 1;
        }
    }
    number_of_wins
}

fn part_one() {
    let contents = fs::read_to_string("./puzzle.txt").unwrap();
    let mut lines: Vec<&str> = contents.lines().collect();
    let distances: Vec<i128> = parse_line(lines.pop().unwrap());
    let times: Vec<i128> = parse_line(lines.pop().unwrap());

    let mut races: Vec<Race> = Vec::new();
    for (current_distance, current_time) in distances.iter().zip(times.iter()) {
        let new_race: Race = Race {
            time: *current_time,
            distance: *current_distance,
        };
        races.push(new_race);
    }
    let mut results: Vec<i128> = Vec::new();
    for race in races {
        results.push(solve_race(race));
    }
    println!("Answer is {}", results.into_iter().product::<i128>());

    println!("Complete");
}

fn part_two() {
    let contents = fs::read_to_string("./puzzle.txt").unwrap();
    let mut lines: Vec<&str> = contents.lines().collect();
    let distance: i128 = parse_line_part_two(lines.pop().unwrap());
    let time: i128 = parse_line_part_two(lines.pop().unwrap());
    let race: Race = Race { distance, time };
    println!("{}", solve_race(race));

    //Alternate way to solve part 2
    // distance_traveled = speed * time_left
    // speed = time_charging
    // time left = total_time - time_charging
    // distance_traveled = time_charging * (total_time - time_charging)
    // distance_traveled = time_charging * total_time - time_charging^2

    let distance_traveled: f64 = race.distance as f64;
    let total_time: f64 = race.time as f64;
}

fn main() {
    //part_one();
    part_two();
}
