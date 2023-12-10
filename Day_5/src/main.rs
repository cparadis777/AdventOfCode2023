use std::collections::VecDeque;
use std::fs;
use std::panic::Location;
use rayon::prelude::*;


#[derive(Clone, Copy)]
struct Mapping {
    source: i128,
    destination: i128,
    range: i128,
}

fn parse_mapping(line: &str) -> Mapping {
    let values: Vec<&str> = line.split(' ').collect();

    Mapping {
        source: values[1].parse::<i128>().unwrap(),
        destination: values[0].parse::<i128>().unwrap(),
        range: values[2].parse::<i128>().unwrap(),
    }
}


fn map_value(value:i128, map:&Mapping) -> Option<i128> {
    let mut result:Option<i128> = Option::None;

    if value >= map.source && value < (map.source + map.range) {
        let offset = value - map.source;
        result = Option::Some(map.destination + offset);
    }

    return result;
}
fn get_mapping(value: i128, maps: &Vec<Mapping>) -> i128 {
    let mut result: Option<i128> = Option::None;
    let current_lower_mapping:i128 = 0;
    for map in maps {
        result = map_value(value, map);
        if let Some(i) = result { result = Option::Some(i); break;}
    }
    match result {
        Some(i) => return i,
        None => return value,
    }
}
#[allow(clippy::eq_op)]
fn part_one() {
    let contents = fs::read_to_string("./puzzle.txt").unwrap();
    let mut lines: VecDeque<&str> = VecDeque::new();
    for line in contents.lines() {
        if !line.is_empty() {
            lines.push_back(line);
        }
    }
    // Parse seeds
    let mut seeds_string: VecDeque<&str> = lines.pop_front().unwrap().split(' ').collect();
    seeds_string.pop_front();
    let mut seeds: Vec<i128> = Vec::new();
    for seed in seeds_string {
        seeds.push(seed.parse::<i128>().unwrap());
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
        let last_limit = block_limits.last().unwrap();
        if limit_idx != block_limits.len()-1 {
            let end_limit = block_limits[limit_idx + 1] - 1;
            for i in (*limit as i128)..(end_limit as i128) {
                let index: usize = (i + 1).try_into().unwrap();
                let mapping = parse_mapping(lines[index]);
                mappings_list.push(mapping);
            }
        } else {
            let end_limit = lines.len() - 1;
            for i in (*limit as i128)..(end_limit as i128) {
                let index: usize = (i + 1).try_into().unwrap();
                let mapping = parse_mapping(lines[index]);
                mappings_list.push(mapping);
            }
        }
        mappings.push_back(mappings_list);
    }
    let mut current_lowest = i128::MAX;
    for seed in seeds {
        //println!("seed: {}", seed);
        let soil = get_mapping(seed, mappings.get(0).unwrap());
        //println!("{}", soil);
        let fertilizer = get_mapping(soil, mappings.get(1).unwrap());
        //println!("{}", fertilizer);
        let water = get_mapping(fertilizer, mappings.get(2).unwrap());
        //println!("{}", water);
        let light = get_mapping(water, mappings.get(3).unwrap());
        //println!("{}", light);
        let temperature = get_mapping(light, mappings.get(4).unwrap());
        //println!("{}", temperature);
        let humidity = get_mapping(temperature, mappings.get(5).unwrap());
        //println!("{}", humidity);
        let location = get_mapping(humidity, mappings.get(6).unwrap());

        println!("seed: {}, location: {}", seed, location);
        if location < current_lowest {
            current_lowest = location;
            println!("Newest lowest location: {}", current_lowest)
        }
    }

    println!("Complete, lowest location: {}", current_lowest);
}

#[allow(clippy::eq_op)]
fn part_two(){
    let contents = fs::read_to_string("./puzzle.txt").unwrap();
    let mut lines: VecDeque<&str> = VecDeque::new();
    for line in contents.lines() {
        if !line.is_empty() {
            lines.push_back(line);
        }
    }
    // Parse seeds
    let mut seeds_string: VecDeque<&str> = lines.pop_front().unwrap().split(' ').collect();
    seeds_string.pop_front();
    let mut seeds: Vec<i128> = Vec::new();
    for seed in seeds_string {
        seeds.push(seed.parse::<i128>().unwrap());  
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
        let last_limit = block_limits.last().unwrap();
        if limit_idx != block_limits.len()-1 {
            let end_limit = block_limits[limit_idx + 1] - 1;
            for i in (*limit as i128)..(end_limit as i128) {
                let index: usize = (i + 1).try_into().unwrap();
                let mapping = parse_mapping(lines[index]);
                mappings_list.push(mapping);
            }
        } else {
            let end_limit = lines.len() - 1;
            for i in (*limit as i128)..(end_limit as i128) {
                let index: usize = (i + 1).try_into().unwrap();
                let mapping = parse_mapping(lines[index]);
                mappings_list.push(mapping);
            }
        }
        mappings.push_back(mappings_list);
    }
    let mut range_starts:Vec<i128> = Vec::new();
    let mut range_sizes:Vec<i128> = Vec::new();
    for (idx, seed) in seeds.into_iter().enumerate() {
        if idx%2 == 0 {
            range_starts.push(seed);
        } else {
            range_sizes.push(seed);
        }

    }
    let mut current_lowest = i128::MAX;
    for (idx, range_start) in range_starts.into_iter().enumerate() {
        for seed in range_start..(range_start + range_sizes[idx]){
            //println!("seed: {}", seed);
            let soil = get_mapping(seed, mappings.get(0).unwrap());
            //println!("{}", soil);
            let fertilizer = get_mapping(soil, mappings.get(1).unwrap());
            //println!("{}", fertilizer);
            let water = get_mapping(fertilizer, mappings.get(2).unwrap());
            //println!("{}", water);
            let light = get_mapping(water, mappings.get(3).unwrap());
            //println!("{}", light);
            let temperature = get_mapping(light, mappings.get(4).unwrap());
            //println!("{}", temperature);
            let humidity = get_mapping(temperature, mappings.get(5).unwrap());
            //println!("{}", humidity);
            let location = get_mapping(humidity, mappings.get(6).unwrap());

            println!("seed: {}, location: {}", seed, location);
            if location < current_lowest {
                current_lowest = location;
                println!("Newest lowest location: {}", current_lowest)
            }
        }
    }

    println!("Complete, lowest location: {}", current_lowest);

}
fn main() {
    //part_one();
    part_two();
}
