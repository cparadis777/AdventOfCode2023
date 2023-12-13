use itertools::Itertools;
use std::fs;

fn get_symbol(map: &Vec<char>, x: usize, y: usize, line_length: usize) -> Option<char> {
    let idx = y * line_length + x;
    map.get(idx).copied()
}

fn get_row(map: &Vec<char>, y: usize, line_length: usize) -> Option<Vec<char>> {
    let result = map.chunks(line_length).nth(y);
    result.map(|x| x.to_vec())
}

fn get_column(map: &Vec<char>, x: usize, line_length: usize) -> Option<Vec<char>> {
    let result = map.chunks(line_length).map(|i| i[x]).collect();
    Option::Some(result)
}

fn taxicab(pos1: (usize, usize), pos2: (usize, usize)) -> usize {
    let p1: (i32, i32) = (pos1.0 as i32, pos1.1 as i32);
    let p2: (i32, i32) = (pos2.0 as i32, pos2.1 as i32);
    let result = (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs();
    result as usize
}

fn main() {
    let contents = fs::read_to_string("./puzzle.txt").unwrap();
    let answer = part_one(&contents).unwrap();
    println!("Answer is: {}", answer);
}

fn part_one(puzzle: &str) -> Option<usize> {
    // Parsing input into a 1D array
    let expand_ratio: usize = 999999;
    let mut result = 0;
    let lines: Vec<&str> = puzzle.lines().collect();
    let n_columns = lines.first().unwrap().len();
    let n_rows = lines.len();
    let mut map: Vec<char> = Vec::new();
    let mut true_x_coordinates: Vec<usize> = Vec::new();
    let mut true_y_coordinates: Vec<usize> = Vec::new();
    let mut galaxies: Vec<(usize, usize)> = Vec::new();
    for line in lines {
        map.append(&mut line.chars().collect::<Vec<char>>());
    }

    // Check all lines and columns to see if they are empty + get coord of the galaxies
    for c_idx in 0..n_columns {
        match get_column(&map, c_idx, n_columns).unwrap().contains(&'#') {
            true => {
                if true_x_coordinates.is_empty() {
                    true_x_coordinates.push(c_idx)
                } else {
                    true_x_coordinates.push(true_x_coordinates.last().unwrap() + 1)
                };
                for (r_idx, char) in get_column(&map, c_idx, n_columns)
                    .unwrap()
                    .iter()
                    .enumerate()
                {
                    if char == &'#' {
                        galaxies.push((c_idx, r_idx));
                    }
                }
            }
            false => {
                if true_x_coordinates.is_empty() {
                    true_x_coordinates.push(c_idx + expand_ratio)
                } else {
                    true_x_coordinates.push(true_x_coordinates.last().unwrap() + expand_ratio + 1)
                };
            }
        }
    }
    for r_idx in 0..n_rows {
        match get_row(&map, r_idx, n_columns).unwrap().contains(&'#') {
            true => {
                if true_y_coordinates.is_empty() {
                    true_y_coordinates.push(r_idx)
                } else {
                    true_y_coordinates.push(true_y_coordinates.last().unwrap() + 1)
                };
            }
            false => {
                if true_y_coordinates.is_empty() {
                    true_y_coordinates.push(r_idx + expand_ratio)
                } else {
                    true_y_coordinates.push(true_y_coordinates.last().unwrap() + expand_ratio + 1)
                };
            }
        }
    }

    let mut sum = 0;
    for pair in galaxies.iter().combinations(2) {
        let p1 = (
            *(true_x_coordinates.get(pair.get(0).unwrap().0)?),
            *(true_y_coordinates.get(pair.get(0).unwrap().1)?),
        );
        let p2 = (
            *(true_x_coordinates.get(pair.get(1).unwrap().0)?),
            *(true_y_coordinates.get(pair.get(1).unwrap().1)?),
        );
        sum += taxicab(p1, p2);
    }
    Option::Some(sum)
}

fn part_two(puzzle: &str) -> Option<i32> {
    let mut result = 0;

    Option::Some(result)
}
