use std::collections::HashMap;
use std::thread::current;
use std::{fs, result};

struct Loop {
    start: Node,
    len: usize,
}

#[derive(Debug, Clone)]
struct Node {
    id: String,
    l: String,
    r: String,
}

impl Node {
    pub fn get_direction(&self, dir: char) -> Option<String> {
        match dir {
            'r' => Option::Some(self.r.clone()),
            'l' => Option::Some(self.l.clone()),
            _ => Option::None,
        }
    }
}

fn parse_node(string: &str) -> Node {
    let mut split_string: Vec<&str> = string.split('=').collect();
    let mut linked_nodes: Vec<&str> = split_string
        .pop()
        .unwrap()
        .trim()
        .trim_matches('(')
        .trim_matches(')')
        .split(',')
        .collect();

    let right = linked_nodes.pop().unwrap().trim().to_string();
    let left = linked_nodes.pop().unwrap().trim().to_string();
    let id = split_string.pop().unwrap().trim().to_string();

    Node {
        id,
        r: right,
        l: left,
    }
}

fn traverse_nodes(
    nodes: HashMap<String, Node>,
    instructions: Vec<char>,
    start: String,
    end: String,
) -> Option<usize> {
    let mut number_of_it: usize = 0;
    let mut current_node: &Node = nodes.get(&start).unwrap();
    let mut instruction_iterator = instructions.iter().cycle();
    while current_node.id != end {
        let current_direction = instruction_iterator.next().unwrap();
        number_of_it += 1;
        match current_direction {
            'L' => {
                current_node = nodes
                    .get(&current_node.get_direction('l').unwrap())
                    .unwrap()
            }
            'R' => {
                current_node = nodes
                    .get(&current_node.get_direction('r').unwrap())
                    .unwrap();
            }
            _ => println!("Invalid direction: {}", current_direction),
        }
        if current_node.id == end {
            return Option::Some(number_of_it);
        }
    }

    Option::Some(instruction_iterator.count())
}

fn find_loop_len(
    nodes: HashMap<String, Node>,
    instructions: Vec<char>,
    start: String,
) -> Option<usize> {
    let mut number_of_it: usize = 0;
    let mut current_node: &Node = nodes.get(&start).unwrap();
    let mut instruction_iterator = instructions.iter().cycle();
    while !current_node.id.ends_with('Z') {
        let current_direction = instruction_iterator.next().unwrap();
        number_of_it += 1;
        match current_direction {
            'L' => {
                current_node = nodes
                    .get(&current_node.get_direction('l').unwrap())
                    .unwrap()
            }
            'R' => {
                current_node = nodes
                    .get(&current_node.get_direction('r').unwrap())
                    .unwrap();
            }
            _ => println!("Invalid direction: {}", current_direction),
        }
        if current_node.id.ends_with('Z') {
            return Option::Some(number_of_it);
        }
    }

    Option::Some(number_of_it)
}
fn main() {
    //part_one();
    part_two();
}

fn part_one() {
    let contents = fs::read_to_string("./puzzle.txt").unwrap();
    let mut lines = contents.lines();
    let mut nodes: HashMap<String, Node> = HashMap::new();
    let mut instructions: Vec<char> = Vec::new();
    for line in lines {
        if !line.is_empty() {
            if line.find('=').is_some() {
                let node: Node = parse_node(line);
                nodes.insert(node.clone().id, node);
            } else {
                instructions = line.chars().collect();
            }
        }
    }
    let start = "AAA".to_string();
    let end = "ZZZ".to_string();
    let result = traverse_nodes(nodes, instructions, start, end).unwrap();
    println!("Complete: {}", result);
}

fn part_two() {
    let contents = fs::read_to_string("./puzzle.txt").unwrap();
    let mut lines = contents.lines();
    let mut nodes: HashMap<String, Node> = HashMap::new();
    let mut instructions: Vec<char> = Vec::new();
    for line in lines {
        if !line.is_empty() {
            if line.find('=').is_some() {
                let node: Node = parse_node(line);
                nodes.insert(node.clone().id, node);
            } else {
                instructions = line.chars().collect();
            }
        }
    }
    let mut start_nodes: Vec<&str> = Vec::new();
    for id in nodes.keys() {
        if id.ends_with('A') {
            start_nodes.push(id);
        }
    }
    let mut loops: Vec<Loop> = Vec::new();
    for start_node in start_nodes {
        let loop_len: usize =
            find_loop_len(nodes.clone(), instructions.clone(), start_node.to_string()).unwrap();
        let new_loop = Loop {
            start: nodes.get(start_node).unwrap().clone(),
            len: loop_len,
        };
        loops.push(new_loop);
    }

    let lens: Vec<usize> = loops.iter().map(|x| x.len).collect();
    let result = lcm(&lens);
    println!("LCM is: {}", result);

    println!("Complete");
}

fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}
