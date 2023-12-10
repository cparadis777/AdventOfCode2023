use std::fs;

fn parse_line(line:&str) -> Vec<i32> {
    line.split(' ').collect::<Vec<&str>>().iter().map(|x| x.parse::<i32>().unwrap()).collect()
}

fn derive_line(line:&Vec<i32>) -> Vec<i32> {
    let mut new_line:Vec<i32> = Vec::new();

    if line.len() > 1{
        for idx in 0..line.len()-1 {
            new_line.push( line.get(idx+1).unwrap() - line.get(idx).unwrap());
        }
    }

    return new_line;
}


fn extrapolate_line(line:Vec<i32>) -> i32 {
    let mut extrapolated_value:i32 = 0;
    let first_value:&i32 = line.get(0).unwrap();
    if line.clone().iter().all(|x| x == first_value){
        extrapolated_value = *first_value;
    } else {
        let change = extrapolate_line(derive_line(&line));
        let new_value = line.last().unwrap() + change;
        extrapolated_value = new_value;
    }
    return extrapolated_value;
}

fn main() {
    //part_one();
    part_two();
}


fn part_one() {

    let contents:String = fs::read_to_string("./puzzle.txt").unwrap();
    let mut parsed_lines:Vec<Vec<i32>> =Vec::new();
    let mut sum:i32 = 0;

    for line in contents.lines() {
        parsed_lines.push(parse_line(line))
    }
    

    for line in parsed_lines{
        sum += extrapolate_line(line);
    }
    

    println!{"Result is: {}", sum};
    println!("Complete");
}

fn part_two(){
    let contents:String = fs::read_to_string("./puzzle.txt").unwrap();
    let mut parsed_lines:Vec<Vec<i32>> =Vec::new();
    let mut sum:i32 = 0;

    for line in contents.lines() {
        let mut parsed_line = parse_line(line);
        parsed_line.reverse();
        parsed_lines.push(parsed_line);
    }
    

    for line in parsed_lines{
        sum += extrapolate_line(line);
    }
    

    println!{"Result is: {}", sum};
    println!("Complete");
}