use std::collections::VecDeque;
use std::fs;
use std::panic::Location;

#[derive(Clone, Copy)]
struct Mapping {
    source: i32,
    destination: i32,
    range: i32,
}

fn parse_mapping(line: &str) -> Mapping {
    let values: Vec<&str> = line.split(' ').collect();

    Mapping {
        source: values[1].parse::<i32>().unwrap(),
        destination: values[0].parse::<i32>().unwrap(),
        range: values[2].parse::<i32>().unwrap(),
    }
}

fn map_value(value: i32, maps: &Vec<Mapping>) -> i32 {
    let mut result: Option<i32> = Option::None;
    for map in maps {
        if value >= map.source && value < (map.source + map.range) {
            let offset = value - map.source;
            result = Some(map.destination + offset);
        }
    }
    match result {
        Some(i) => return i,
        None => return value,
    }
}
#[allow(clippy::eq_op)]
fn part_one() {
    let contents = fs::read_to_string("./sample.txt").unwrap();
    let mut lines: VecDeque<&str> = VecDeque::new();
    for line in contents.lines() {
        if !line.is_empty() {
            lines.push_back(line);
        }
    }
    // Parse seeds
    let mut seeds_string: VecDeque<&str> = lines.pop_front().unwrap().split(' ').collect();
    seeds_string.pop_front();
    let mut seeds: Vec<i32> = Vec::new();
    for seed in seeds_string {
        seeds.push(seed.parse::<i32>().unwrap());
    }
    // parse map blocks
    let mut block_limits: Vec<usize> = Vec::new();
    for (line_idx, line) in lines.iter().enumerate() {
        let match_idx = line.find("map");
        if let Some(_i) = match_idx {
            block_limits.push(line_idx)
        };
    }
    // parse individual map lines
    let mut mappings: VecDeque<Vec<Mapping>> = VecDeque::new();
    for (limit_idx, limit) in block_limits.clone().iter().enumerate() {
        let mut mappings_list: Vec<Mapping> = Vec::new();
        if limit != block_limits.last().unwrap() {
            let end_limit = block_limits[limit_idx + 1] - 1;
            for i in (*limit as i32)..(end_limit as i32) {
                let index: usize = (i + 1).try_into().unwrap();
                let mapping = parse_mapping(lines[index]);
                mappings_list.push(mapping);
            }
        } else {
        }
        mappings.push_back(mappings_list);
    }

    for seed in seeds {
        println!("seed: {}", seed);
        let soil = map_value(seed, mappings.get(0).unwrap());
        println!("{}", soil);
        let fertilizer = map_value(soil, mappings.get(1).unwrap());
        println!("{}", fertilizer);
        let water = map_value(fertilizer, mappings.get(2).unwrap());
        println!("{}", water);
        let light = map_value(water, mappings.get(3).unwrap());
        println!("{}", light);
        let temperature = map_value(light, mappings.get(4).unwrap());
        println!("{}", temperature);
        let humidity = map_value(temperature, mappings.get(5).unwrap());
        println!("{}", humidity);
        let location = map_value(humidity, mappings.get(6).unwrap());

        println!("seed: {}, location: {}", seed, location);
    }

    println!("Complete");
}

fn main() {
    part_one();
}
