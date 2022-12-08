use std::fs::File;
use std::io::Read;
use std::path::Path;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_unique_chars() {
        let signal = String::from("abcd");
        assert_eq!(check_unique_chars(&signal), true);
        let signal = String::from("aacd");
        assert_eq!(check_unique_chars(&signal), false);
        let signal = String::from("abdd");
        assert_eq!(check_unique_chars(&signal), false);
        let signal = String::from("abca");
        assert_eq!(check_unique_chars(&signal), false);
    }

    #[test]
    fn test_part1() {
        let fname = String::from("data/test_input");
        let result = solve_part1(&fname);
        assert_eq!(result, 7);
    }

    #[test]
    fn test_part2() {
        let fname = String::from("data/test_input");
        let result = solve_part2(&fname);
        assert_eq!(result, 19);
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

fn check_unique_chars(subset: &str) -> bool {
    for i in 0..subset.len() {
        for j in i + 1..subset.len() {
            if subset.chars().nth(i) == subset.chars().nth(j) {
                return false;
            }
        }
    }
    true
}

fn solve_part1(fname: &String) -> u32 {
    // Read data file
    let file_content = read_file(&fname);
    // Convert the signal into a vec
    let signal = file_content.replace("\n", "");
    // Find the marker position
    let mut position: u32 = 0;
    for i in (4 - 1)..signal.len() {
        // Check if there are non-repeated characters in the current
        // sequence of four elements in the signal
        if check_unique_chars(&signal[i - (4 - 1)..i + 1]) {
            // Found marker!
            // (need to add 1 because in Rust indices start in zero)
            position = i as u32 + 1;
            break;
        }
    }
    // Return marker position
    position
}

fn solve_part2(fname: &String) -> u32 {
    // Read data file
    let file_content = read_file(&fname);
    // Convert the signal into a vec
    let signal = file_content.replace("\n", "");
    // Find the marker position
    let mut position: u32 = 0;
    for i in (14 - 1)..signal.len() {
        // Check if there are non-repeated characters in the current
        // sequence of four elements in the signal
        if check_unique_chars(&signal[i - (14 - 1)..i + 1]) {
            // Found marker!
            // (need to add 1 because in Rust indices start in zero)
            position = i as u32 + 1;
            break;
        }
    }
    // Return marker position
    position
}

fn main() {
    let fname = String::from("data/input");
    // let fname = String::from("data/test_input");

    // part 1
    let result = solve_part1(&fname);
    println!("Solution to part 1: {}", result);

    // part 2
    let result = solve_part2(&fname);
    println!("Solution to part 2: {}", result);
}
