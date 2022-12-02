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
        assert_eq!(result, 24000);
    }

    #[test]
    fn test_part2() {
        let fname = String::from("data/test_input");
        let result = solve_part2(&fname);
        assert_eq!(result, 45000);
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
    let mut data_str = String::new();
    match file.read_to_string(&mut data_str) {
        Err(why) => panic!("couldn't read {}: {}", fname, why),
        Ok(_) => (),
    };
    data_str
}

fn solve_part1(fname: &String) -> u32 {
    // Read data file
    let data = read_file(&fname);
    // Find out how many calories are being carried by the Elf that carries
    // the most calories
    let mut highest_calories: u32 = 0;
    let mut calories: u32 = 0;
    for line in data.lines() {
        if line.trim().is_empty() {
            if highest_calories < calories {
                highest_calories = calories;
            }
            calories = 0;
        } else {
            calories += line.parse::<u32>().unwrap();
        }
    }
    // Check again for the last set of calories in the file
    if highest_calories < calories {
        highest_calories = calories;
    }
    highest_calories
}

fn solve_part2(fname: &String) -> u32 {
    // Read data file
    let data = read_file(&fname);
    // Find out how many calories are being carried by the Elf that carries
    // the most calories
    let mut first: u32 = 0;
    let mut second: u32 = 0;
    let mut third: u32 = 0;
    let mut calories: u32 = 0;
    for line in data.lines() {
        if line.trim().is_empty() {
            if calories > first {
                third = second;
                second = first;
                first = calories;
            } else if calories > second {
                third = second;
                second = calories;
            } else if calories > third {
                third = calories;
            }
            calories = 0;
        } else {
            calories += line.parse::<u32>().unwrap();
        }
    }
    // Check again for the last set of calories in the file
    if calories > first {
        third = second;
        second = first;
        first = calories;
    } else if calories > second {
        third = second;
        second = calories;
    } else if calories > third {
        third = calories;
    }
    first + second + third
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
