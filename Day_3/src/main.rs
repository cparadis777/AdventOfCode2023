use std::fs;



fn spread_sideways(puzzle:&Vec<Vec<char>>, line_idx:usize, char_idx:usize) -> i32 {
    let mut number_string:String= "".to_string();
    let mut current_idx:usize = char_idx;
    let mut current_char:char = puzzle[line_idx][current_idx];

    // spread left
    loop {
        if current_idx != 0 { 
            let next_id = current_idx-1;
            let next_character:char = puzzle[line_idx][next_id];
            if next_character.is_numeric() {
                current_char = next_character;
                current_idx = next_id;
            } else {
                break;
            }
        } else {
            break;
        }
    }
    //spread right
    while current_char.is_numeric() {
        current_idx += 1;
        number_string = [number_string, current_char.to_string()].join("");
        if current_idx >= puzzle[line_idx].len()-1 {
            break
            }
        current_char = puzzle[line_idx][current_idx];
        }


    let number = number_string.parse::<i32>();
    match number{
        Ok(result) => return result,
        Err(_e) => return 0,
    }
}

fn main() {

    let contents:String = fs::read_to_string("sample.txt").unwrap();
    let mut puzzle:Vec<Vec<char>> =  Vec::new();
    for line in contents.lines() {
        let mut current_line:Vec<char> = Vec::new();
        for character in line.chars(){
            current_line.push(character);
        }
        puzzle.push(current_line.clone());
        
    }
    let mut sum:i32 = 0;
    for (line_idx, line) in puzzle.iter().enumerate() {
        println!("Checking line #{}: {:?}", line_idx+1, line);
        for (char_idx, character) in line.iter().enumerate() {
            if character.is_ascii_punctuation() {
                if character != &'.' {
                    println!("Found symbol {} at index {}", character, char_idx);
                    //Check left
                    let left_number:i32 = spread_sideways(&puzzle, line_idx, char_idx-1);
                    //Check right
                    let right_number:i32 = spread_sideways(&puzzle, line_idx, char_idx+1);
                    //Check up
                    let mut up_number:i32 = 0;
                    if line_idx != 0{
                        up_number = spread_sideways(&puzzle, line_idx-1, char_idx);
                    }
                    //Check down
                    let mut down_number:i32 = 0;
                    if line_idx != puzzle.len()-1{
                        down_number = spread_sideways(&puzzle, line_idx+1, char_idx);
                    }
                    
                    println!("left: {}, right: {}, up: {}, down: {}", left_number, right_number, up_number, down_number);
                    sum = sum + left_number + right_number + up_number + down_number;
                }

            }
        }
        println!("{}", sum);
    }

}
