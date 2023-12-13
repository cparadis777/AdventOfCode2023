use ndarray::{s, Array, Array2, ArrayBase, Axis, Dimension, Slice};
use std::{cmp, fs};

fn parse_pattern(pattern: &str) -> Option<Array2<char>> {
    let lines: Vec<&str> = pattern.lines().collect();
    let dim: (usize, usize) = (lines.len() - 0, lines.first()?.len() - 0);
    let mut array: Array2<char> = Array::from_elem(dim, '-');
    for i in 0..dim.0 {
        let chars: Vec<char> = lines[i].chars().collect();
        for j in 0..dim.1 {
            array[[i, j]] = chars[j];
        }
    }
    Option::Some(array)
}

fn check_symmetry(pattern: &Array2<char>, axis: usize) -> bool {
    let n_columns_right = pattern.len_of(Axis(1)) - axis;
    let n_columns_left = axis;
    let width = cmp::min(n_columns_left, n_columns_right);
    let left_columns = pattern.slice(s![(axis - width)..axis, ..]);
    let right_columns = pattern.slice(s![axis..(axis + width);-1, ..]);
    if left_columns == right_columns {
        println!("ding");
    }
    left_columns == right_columns
}

fn main() {
    let contents = fs::read_to_string("./sample.txt").unwrap();
    let result = part_one(contents);
    //let result = part_two(contents);
    match result {
        Some(x) => println!("{}", x),
        None => println!("Couldn't find solution"),
    }
}

fn part_one(contents: String) -> Option<usize> {
    let mut patterns_string: Vec<&str> = contents
        .split("\r\n\r\n")
        .filter(|x| !x.is_empty())
        .collect();

    let mut patterns: Vec<Array2<char>> = Vec::new();
    for line in patterns_string {
        patterns.push(parse_pattern(line)?);
    }
    for pattern in patterns.iter() {
        for axis in 1..pattern.len_of(Axis(1)) - 1 {
            if check_symmetry(pattern, axis) {
                println!("Symmetry found at axis {}", axis);
            }
        }
    }
    Option::None
}

fn part_two(contents: String) -> Option<usize> {
    Option::None
}
