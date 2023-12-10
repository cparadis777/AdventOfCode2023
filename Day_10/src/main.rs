use std::fs;
use std::collections::HashMap;
use colored::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Position {
    x:i32,
    y:i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {Up, Right, Down, Left}
impl Direction{
    fn delta(&self) -> (i32, i32){
        match *self {
            Direction::Up => (0,-1),
            Direction::Right => (1,0),
            Direction::Down => (0,1),
            Direction::Left => (-1,0),
        }
    }
    fn opposite(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
        }
    }
}

fn get_pipe_exit(pipe:&char, entrance_direction:Direction) -> Option<Direction>{
    match pipe {
        '─' => match entrance_direction {
                Direction::Left => Option::Some(Direction::Right),
                Direction::Right => Option::Some(Direction::Left),
                _ => Option::None
            },
        '│' =>  match entrance_direction {
                Direction::Up => Option::Some(Direction::Down),
                Direction::Down => Option::Some(Direction::Up),
                _ => Option::None
            },
        '┌' =>  match entrance_direction {
                Direction::Down => Option::Some(Direction::Right),
                Direction::Right => Option::Some(Direction::Down),
                _ => Option::None
            },
        '┐' =>  match entrance_direction {
                Direction::Left => Option::Some(Direction::Down),
                Direction::Down => Option::Some(Direction::Left),
                _ => Option::None
            },
        '└' =>  match entrance_direction {
                Direction::Up => Option::Some(Direction::Right),
                Direction::Right => Option::Some(Direction::Up),
                _ => Option::None
            },
        '┘' =>  match entrance_direction {
                Direction::Left => Option::Some(Direction::Up),
                Direction::Up => Option::Some(Direction::Left),
                _ => Option::None
            },
        _ => Option::None,
    }
}

fn get_symbol(map: &Vec<Vec<char>>, coordinate:Position) -> Option<char> {
    let line_option:Option<&Vec<char>> = map.get(coordinate.y as usize);
    let line:&Vec<char> = match line_option {
        Some(x) => x,
        _ => return Option::None,
    };
    let pipe_option:Option<&char> = line.get(coordinate.x as usize);
    let pipe = match pipe_option {
        Some(x) => x,
        _ => return Option::None,
    };
    return Option::Some(*pipe);
}

fn find_start_direction(map:&Vec<Vec<char>>, start_position:Position) -> Option<Direction>{
    let dir_list:Vec<Direction> = Vec::from([Direction::Up, Direction::Right, Direction::Down, Direction::Left]);
    let current_position:Position = start_position;
    for direction in dir_list{
        let delta:(i32, i32) = direction.delta();
        let next_coord:Position = Position{x: current_position.x + delta.0, y: current_position.y + delta.1};
        if let Some(_exit) = get_pipe_exit(&get_symbol(map, next_coord)?, direction.opposite()) {
            let start_direction = direction;
            return Option::Some(start_direction);
        }
    }
    return Option::None;
}

fn move_to_next_pipe(map:&Vec<Vec<char>>, current_position:Position, entrance:Direction ) -> Option<Position> {
    let delta:(i32, i32) = entrance.delta();
    let next_coord:Position = Position { x: current_position.x + delta.0, y: current_position.y + delta.1 };
    //println!("Moving to pipe {} at position {:?} from {:?}", get_symbol(map, next_coord)?, next_coord, entrance);

    Option::Some(next_coord)
}

fn traverse_pipes(map:&Vec<Vec<char>>, start_position:Position) -> Option<i32> {
    let start_direction:Direction = find_start_direction(map, start_position)?;
    let mut distance:i32 = 0;
    let mut exit_direction:Direction = start_direction;
    let mut current_position:Position = start_position;
    while true {
        //Direct move to next node
        distance += 1;
        current_position = move_to_next_pipe(map, current_position, exit_direction)?;
        let test= 0;
        if get_symbol(map, current_position)? == 'S' {
            return Option::Some(distance);
        }
        // Check which way we exit that pipe

        let x = 1+1;
        let next_character:&char = &get_symbol(map, current_position)?;
        exit_direction = get_pipe_exit(next_character, exit_direction.opposite())?;
       
    }
    Option::None
}

fn traverse_pipes_part_two(map:&Vec<Vec<char>>, start_position:Position) -> Option<HashMap<(i32, i32), Direction>> {
    let start_direction:Direction = find_start_direction(map, start_position)?;
    let mut distance:i32 = 0;
    let mut exit_direction:Direction = start_direction;
    let mut current_position:Position = start_position;
    let mut history:HashMap<(i32, i32), Direction> = HashMap::new();

    while true {
        //Direct move to next node
        history.insert((current_position.x, current_position.y), exit_direction);
        distance += 1;
        current_position = move_to_next_pipe(map, current_position, exit_direction)?;
  
        let test= 0;
        if get_symbol(map, current_position)? == 'S' {
            return Option::Some(history);
        }
        // Check which way we exit that pipe

        let x = 1+1;
        let next_character:&char = &get_symbol(map, current_position)?;
        exit_direction = get_pipe_exit(next_character, exit_direction.opposite())?;
       
    }
    Option::None
}

fn find_start(map:&Vec<Vec<char>>) -> Option<Position> {
    for (i, line) in map.into_iter().enumerate(){
        for (j, symbol) in line.into_iter().enumerate() {
            if *symbol == 'S' {
                return Option::Some(Position{x:j as i32, y:i as i32})
            }
        }
    }
    return Option::None
}



fn part_one() {
    let contents = fs::read_to_string("./sample.txt").unwrap();
    let contents = fs::read_to_string("./puzzle.txt").unwrap();
    let map:Vec<Vec<char>> = contents.lines()
    .map(|x| x.chars()
    .map(|c| match c {
        '-' => '─',
        '|' => '│',
        'F' => '┌',
        '7' => '┐',
        'L' => '└',
        'J' => '┘',
        _ => c,
     })
    .collect::<Vec<char>>())
    .collect();
    let _ = fs::write("./test.txt", contents.chars().map(|c| match c {
        '-' => '─',
        '|' => '│',
        'F' => '┌',
        '7' => '┐',
        'L' => '└',
        'J' => '┘',
        _ => c,
     }).collect::<String>());
    /*
    let test = find_start(&map).unwrap();
    println!("{:?}: {:?}", get_symbol(&map, test).unwrap(), test);
    let test1 = Position{x:test.x, y:test.y+1};
    let test2 = Position{x:test1.x, y:test1.y+1};
    let test3:Position =Position { x: 1, y: 2 };
    println!("{:?}: {:?}", get_symbol(&map, test1).unwrap(), test1);
    println!("{:?}: {:?}", get_symbol(&map, test2).unwrap(), test2);
    println!("{:?}: {:?}", get_symbol(&map, test3).unwrap(), test3);
    */

    let debug_center:Position = Position { x: 101, y: 96 };

    let result:Option<i32> = traverse_pipes(&map, find_start(&map).unwrap());
    let mut n_of_steps:i32 = 0;
    match result{
        Some(x) => n_of_steps = x,
        _ => {
            let mut debug_view:Vec<String> = Vec::new();
            for line in -3 ..3 {
                let line_idx = (debug_center.y  + line) as usize;
                let debug_line = map.get(line_idx).unwrap();
                let debug_start = (debug_center.x - 3) as usize;
                let debug_end = (debug_center.x + 3) as usize;
                let mut debug_slice:String= debug_line.get(debug_start..debug_end).unwrap().iter().collect::<String>();
                //debug_slice.push_str("\n");
                debug_view.push(debug_slice);
            }
            let debug_string:String = debug_view.iter().map(|x| x.to_string()).collect();
            for line in debug_view {
                println!("{}", line);
            }
            println!("Done");
        },
    }
    n_of_steps = n_of_steps/2;
    println!("{}", n_of_steps);

}

fn part_two(){
    let contents = fs::read_to_string("./puzzle.txt").unwrap();
    //let contents = fs::read_to_string("./puzzle.txt").unwrap();
    let map:Vec<Vec<char>> = contents.lines()
    .map(|x| x.chars()
    .map(|c| match c {
        '-' => '─',
        '|' => '│',
        'F' => '┌',
        '7' => '┐',
        'L' => '└',
        'J' => '┘',
        _ => c,
     })
    .collect::<Vec<char>>())
    .collect();

    let mut sum:i32 = 0;
    let mut parity = 0;
    let result:Option<HashMap<(i32, i32), Direction>> = traverse_pipes_part_two(&map, find_start(&map).unwrap());
    let history:HashMap<(i32, i32), Direction> = result.unwrap();
    let mut reduced_map:Vec<Vec<char>> = Vec::new();
    let number_of_lines = map.len();
    let number_of_char = map.get(0).unwrap().len();
    for j in 0..number_of_lines{
        let mut new_vec:Vec<char> = Vec::new();
        for i in 0..number_of_char{
            if  history.get(&(i as i32,j as i32)).is_some() {
                new_vec.push(*map.get(j).unwrap().get(i).unwrap());
            } else {
                new_vec.push('.');
            }
        }
        reduced_map.push(new_vec);
    }

    for j in 0..number_of_lines{
        parity = 0;
        for i in 0..number_of_char{
            let current_char:&char= reduced_map.get(j).unwrap().get(i).unwrap();
            match *current_char {
                '│' | '┘' |'└' |'S' => {
                    parity += 1;
                    if parity%2 == 1 {
                        print!("{}", String::from(*current_char).red());
                    }else{
                        print!("{}", String::from(*current_char).blue());
                    }
                }
                '.' => if parity%2 == 1 {
                        print!("{}", String::from(*current_char).green());
                        sum += 1;
                        } else {
                            print!("{}", String::from(*current_char).white());
                        },
                _ => { if parity%2 == 1 {
                            print!("{}", String::from(*current_char).red());
                        }else {
                            print!("{}", String::from(*current_char).blue());
                        }
                    },
                }
        }
        print!("\n");
    }
    println!("{}", sum);
}

fn main() {
    //part_one();
    part_two();
}
