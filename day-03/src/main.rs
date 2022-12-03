use std::fs::File;
use std::io::Read;
use std::path::Path;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_priority() {
        let item_types = ['a', 'A', 'z', 'Z'];
        let expected_priorities = [1, 27, 26, 52];
        for (item_type, expected_priority) in
            item_types.into_iter().zip(expected_priorities.into_iter())
        {
            assert_eq!(compute_priority(&item_type), expected_priority);
        }
    }

    #[test]
    fn test_repeated_item_type() {
        assert_eq!(find_repeated_item_type("abcdaf"), 'a');
        assert_eq!(find_repeated_item_type("abcdec"), 'c');
    }

    #[test]
    fn test_part1() {
        let fname = String::from("data/test_input");
        let result = solve_part1(&fname);
        assert_eq!(result, 157);
    }

    // #[test]
    // fn test_part2() {
    //     let fname = String::from("data/test_input");
    //     let result = solve_part2(&fname);
    //     assert_eq!(result, 12);
    // }
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

fn find_repeated_item_type(rucksack: &str) -> char {
    // Find the repeated item type in the two compartiments of the rucksack
    let len: usize = rucksack.len();
    let first = &rucksack[..len / 2];
    let second = &rucksack[len / 2..len];
    for item in first.chars() {
        if second.contains(item) {
            return item as char;
        }
    }
    panic!("No repeated item type was found in {}", rucksack);
}

fn compute_priority(item_type: &char) -> u32 {
    // Compute the priority of an item type
    if item_type.is_lowercase() {
        return (*item_type as u32) - 96;
    } else {
        return (*item_type as u32) - 64 + 26;
    }
}

fn solve_part1(fname: &String) -> u32 {
    // Read data file
    let data = read_file(&fname);
    let mut priorities: u32 = 0;
    for line in data.lines() {
        let repeated_item_type = find_repeated_item_type(&line);
        priorities += compute_priority(&repeated_item_type);
    }
    priorities
}

fn main() {
    let fname = String::from("data/input");

    // part 1
    let result = solve_part1(&fname);
    println!("Solution to part 1: {}", result);

    // part 2
    // let result = solve_part2(&fname);
    // println!("Solution to part 2: {}", result);
}
