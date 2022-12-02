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

fn update_highest_calories(highest_calories: &mut u32, calories: &u32) {
    // Update the highest calories if calories is greater than it
    if *highest_calories < *calories {
        *highest_calories = *calories;
    }
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
            update_highest_calories(&mut highest_calories, &calories);
            calories = 0;
        } else {
            calories += line.parse::<u32>().unwrap();
        }
    }
    // Check again for the last set of calories in the file
    update_highest_calories(&mut highest_calories, &calories);
    highest_calories
}

fn update_top_three_calories(top_three: &mut [u32; 3], calories: &u32) {
    // Update the top three highest calories
    if *calories > top_three[0] {
        top_three[2] = top_three[1];
        top_three[1] = top_three[0];
        top_three[0] = *calories;
    } else if *calories > top_three[1] {
        top_three[2] = top_three[1];
        top_three[1] = *calories;
    } else if *calories > top_three[2] {
        top_three[2] = *calories;
    }
}

fn solve_part2(fname: &String) -> u32 {
    // Read data file
    let data = read_file(&fname);
    // Find out how many calories are being carried by the Elf that carries
    // the most calories
    let mut top_three: [u32; 3] = [0; 3];
    let mut calories: u32 = 0;
    for line in data.lines() {
        if line.trim().is_empty() {
            update_top_three_calories(&mut top_three, &calories);
            calories = 0;
        } else {
            calories += line.parse::<u32>().unwrap();
        }
    }
    // Check again for the last set of calories in the file
    update_top_three_calories(&mut top_three, &calories);
    top_three.iter().sum()
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
