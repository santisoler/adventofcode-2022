use std::fs::File;
use std::io::Read;
use std::path::Path;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let fname = String::from("data/test_input");
        let result = solve_part1(&fname);
        assert_eq!(result, "CMZ");
    }

    #[test]
    fn test_part2() {
        let fname = String::from("data/test_input");
        let result = solve_part2(&fname);
        assert_eq!(result, "MCD");
    }
}

fn read_file(fname: &String) -> String {
    // Open file
    let path = Path::new(&fname);
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", fname, why),
        Ok(file) => file,
    };
    // Parse file
    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Err(why) => panic!("couldn't read {}: {}", fname, why),
        Ok(_) => (),
    };
    content
}

fn parse_location_of_crates(file_content: &str) -> Vec<Vec<char>> {
    let n_crates = _get_number_of_crates(&file_content);
    let mut crates: Vec<Vec<char>> = vec![[].to_vec(); n_crates as usize];
    for line in file_content.lines() {
        if line.chars().nth(1).expect("bla").is_digit(10) {
            break;
        }
        for i in 0..(line.len() - 2) / 4 + 1 {
            let char_index = 4 as usize * i as usize + 1;
            let char = line.chars().nth(char_index).expect("bla");
            if char.is_alphabetic() {
                crates[i].insert(0, char)
            };
        }
    }
    crates
}

fn _get_number_of_crates(file_content: &str) -> u32 {
    let mut n_crates: u32 = 0;
    for line in file_content.lines() {
        if !line.is_empty() {
            if &line.trim()[0..1] == "1" {
                n_crates = line
                    .split_whitespace()
                    .last()
                    .unwrap()
                    .parse()
                    .expect("Failed to parse line");
                break;
            }
        }
    }
    n_crates
}

fn parse_move(instructions: &str) -> (u32, usize, usize) {
    let moves = instructions.split_whitespace().collect::<Vec<&str>>();
    (
        moves[1].parse().expect("cannot parse"),
        moves[3].parse().expect("cannot parse"),
        moves[5].parse().expect("cannot parse"),
    )
}

fn solve_part1(fname: &String) -> String {
    // Read data file
    let file_content = read_file(&fname);
    let mut crates = parse_location_of_crates(&file_content);
    let mut read_move = false;
    for line in file_content.lines() {
        if line.is_empty(){
            read_move = true;
            continue
        }
        if read_move {
            let movements = parse_move(&line);
            for _ in 0..movements.0 {
                let tmp = crates[movements.1 - 1].pop().expect("Cannot pop crate");
                crates[movements.2 - 1].push(tmp);
            };
        }
    }
    let mut message: Vec<char> = Vec::new();
    for column in crates.iter() {
        message.push(column.last().expect("no last element").clone())
    }
    message.iter().collect::<String>()
}

fn solve_part2(fname: &String) -> String {
    // Read data file
    let file_content = read_file(&fname);
    let mut crates = parse_location_of_crates(&file_content);
    let mut read_move = false;
    for line in file_content.lines() {
        if line.is_empty(){
            read_move = true;
            continue
        }
        if read_move {
            let movements = parse_move(&line);
            let mut tmp: Vec<char> = Vec::new();
            for _ in 0..movements.0 {
                tmp.push(crates[movements.1 - 1].pop().expect("Cannot pop crate"));
            };
            tmp.reverse();
            crates[movements.2 - 1].extend(tmp);
        }
    }
    let mut message: Vec<char> = Vec::new();
    for column in crates.iter() {
        message.push(column.last().expect("no last element").clone())
    }
    message.iter().collect::<String>()
}

fn main() {
    let fname = String::from("data/input");

    // part 1
    let result = solve_part1(&fname);
    println!("Solution to part 1: {}", result);

    // part 2
    let result = solve_part2(&fname);
    println!("Solution to part 2: {}", result);
}
