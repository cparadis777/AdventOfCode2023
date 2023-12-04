use std::collections::HashMap;
use std::fs;

fn main() {
    let search_strings = HashMap::from([
        ("1", 1),
        ("one", 1),
        ("2", 2),
        ("two", 2),
        ("3", 3),
        ("three", 3),
        ("4", 4),
        ("four", 4),
        ("5", 5),
        ("five", 5),
        ("6", 6),
        ("six", 6),
        ("7", 7),
        ("seven", 7),
        ("8", 8),
        ("eight", 8),
        ("9", 9),
        ("nine", 9),
    ]);

    let contents = fs::read_to_string("./puzzle.txt").unwrap();
    let lines: Vec<&str> = contents.split('\n').collect();
    let mut numbers: Vec<i32> = Vec::new();
    let max_char = 5;

    for line in lines.iter() {
        if line.is_empty() {
            break;
        }
        let mut matches: HashMap<usize, i32> = HashMap::new();

        for (pattern, value) in search_strings.iter() {
            for (idx, _matched) in line.match_indices(pattern) {
                matches.insert(idx, *value);
            }
        }

        let mut number1: i32 = 0;
        let mut number2: i32 = 0;

        if matches.is_empty() {
            panic!("No numbers were found in line {}", line);
        } else {
            let indices: Vec<_> = matches.keys().collect();
            let idx1 = indices.iter().min().unwrap();
            let idx2 = indices.iter().max().unwrap();
            number1 = *matches.get(idx1).unwrap();
            number2 = *matches.get(idx2).unwrap();
            numbers.push(number1 * 10 + number2);
        }
    }
    let mut sum: i32 = 0;
    for number in numbers {
        sum += number;
    }
    println!("{:?}", sum);
}
