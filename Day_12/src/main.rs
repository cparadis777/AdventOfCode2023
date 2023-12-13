use std::collections::HashMap;
use std::fs;

fn calculate_solutions(
    springs: &Vec<char>,
    constraints: &Vec<u128>,
    memoization: &mut HashMap<(Vec<u128>, Vec<char>), u128>,
) -> u128 {
    if springs.is_empty() {
        if constraints.is_empty() {
            return 1;
        } else {
            return 0;
        }
    }

    match springs[0] {
        '.' => calculate_solutions(&springs[1..].to_vec(), constraints, memoization),
        '#' => calculate_hash_solutions(springs, constraints, memoization),
        '?' => {
            calculate_solutions(&springs[1..].to_vec(), constraints, memoization)
                + calculate_hash_solutions(springs, constraints, memoization)
        }
        _ => panic!("Unexpected character in puzzle definition"),
    }
}

fn calculate_hash_solutions(
    springs: &Vec<char>,
    constraints: &Vec<u128>,
    memoization: &mut HashMap<(Vec<u128>, Vec<char>), u128>,
) -> u128 {
    if let Some(&result) = memoization.get(&(constraints.clone(), springs.clone())) {
        return result;
    }
    if constraints.is_empty() {
        return 0;
    }
    let x = constraints[0] as usize;
    if springs.len() < x {
        return 0;
    }

    for i in 0..x {
        if springs[i] == '.' {
            return 0;
        }
    }

    if springs.len() == x {
        if constraints.len() == 1 {
            return 1;
        }
        return 0;
    }

    if springs[x] == '#' {
        return 0;
    }

    let result = calculate_solutions(
        &springs[(x + 1)..].to_vec(),
        &constraints[1..].to_vec(),
        memoization,
    );
    memoization.insert((constraints.clone(), springs.clone()), result);
    result
}

fn parse_puzzle(line: &str) -> Option<(Vec<char>, Vec<u128>)> {
    let mut sections: Vec<&str> = line.split(' ').collect();

    let constraints = sections
        .pop()?
        .chars()
        .filter(|x| x != &',')
        .map(|x| x.to_digit(10).unwrap() as u128)
        .collect();
    let springs = sections.pop()?.chars().collect();

    Option::Some((springs, constraints))
}

fn main() {
    let contents = fs::read_to_string("./puzzle.txt").unwrap();
    //let result: Option<u128> = part_one(contents);
    let result: Option<u128> = part_two(contents);

    match result {
        Some(x) => println!("Result is {}", x),
        None => println!("Couldn't solve problem"),
    }
}

fn part_one(contents: String) -> Option<u128> {
    // Parse the input -> define struct for each individual puzzle
    let mut puzzles: Vec<(Vec<char>, Vec<u128>)> = Vec::new();
    let mut memoization: HashMap<(Vec<u128>, Vec<char>), u128> = HashMap::new();
    for line in contents.lines() {
        puzzles.push(parse_puzzle(line)?);
    }
    // Solve individual puzzles
    let total: u128 = puzzles
        .iter()
        .map(|(springs, constraints)| calculate_solutions(springs, constraints, &mut memoization))
        .sum();
    // Sum all the puzzle's # of solutions
    Option::Some(total)
}

fn part_two(contents: String) -> Option<u128> {
    let mut data_rows = Vec::new();
    let mut memoization = HashMap::new();

    for line in contents.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let springs: Vec<&str> = parts[0].split('?').collect();
        let groups: Vec<&str> = parts[1].split(',').collect();

        let new_springs: String = springs
            .iter()
            .cycle()
            .take(springs.len() * 5)
            .cloned()
            .collect::<Vec<&str>>()
            .join("?");
        let new_groups: String = groups
            .iter()
            .cycle()
            .take(groups.len() * 5)
            .cloned()
            .collect::<Vec<&str>>()
            .join(",");

        let springs_chars: Vec<char> = new_springs.chars().collect();
        let groups_int: Vec<u128> = new_groups.split(',').map(|s| s.parse().unwrap()).collect();

        data_rows.push((springs_chars, groups_int));
    }

    let total: u128 = data_rows
        .iter()
        .map(|(springs, groups)| calculate_solutions(springs, groups, &mut memoization))
        .sum();

    Option::Some(total)
}
