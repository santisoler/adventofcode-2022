use std::fs::File;
use std::io::Read;
use std::path::Path;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_lines() {
        let line = String::from("1-2,3-4");
        assert_eq!(parse_line(&line), vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_check_fully_contained_pairs() {
        let pairs: Vec<u32> = Vec::from([1, 2, 3, 4]);
        assert_eq!(check_pairs_fully_contained(&pairs), false);
        let pairs: Vec<u32> = Vec::from([1, 4, 3, 5]);
        assert_eq!(check_pairs_fully_contained(&pairs), false);
        let pairs: Vec<u32> = Vec::from([2, 6, 2, 4]);
        assert_eq!(check_pairs_fully_contained(&pairs), true);
        let pairs: Vec<u32> = Vec::from([8, 9, 6, 9]);
        assert_eq!(check_pairs_fully_contained(&pairs), true);
    }

    #[test]
    fn test_check_overlap_pairs() {
        let pairs: Vec<u32> = Vec::from([1, 2, 3, 4]);
        assert_eq!(check_pairs_overlap(&pairs), false);
        let pairs: Vec<u32> = Vec::from([6, 8, 3, 4]);
        assert_eq!(check_pairs_overlap(&pairs), false);
        let pairs: Vec<u32> = Vec::from([1, 4, 4, 5]);
        assert_eq!(check_pairs_overlap(&pairs), true);
        let pairs: Vec<u32> = Vec::from([4, 5, 3, 5]);
        assert_eq!(check_pairs_overlap(&pairs), true);
    }

    #[test]
    fn test_part1() {
        let fname = String::from("data/test_input");
        let result = solve_part1(&fname);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_part2() {
        let fname = String::from("data/test_input");
        let result = solve_part2(&fname);
        assert_eq!(result, 4);
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

fn parse_line(line: &str) -> Vec<u32> {
    let mut pairs: Vec<u32> = Vec::new();
    for pair in line.split(',') {
        for bound in pair.split('-') {
            pairs.push(bound.parse().unwrap());
        }
    }
    pairs
}

fn check_pairs_fully_contained(pairs: &Vec<u32>) -> bool {
    if (pairs[0] <= pairs[2]) & (pairs[1] >= pairs[3]) {
        return true;
    }
    if (pairs[2] <= pairs[0]) & (pairs[3] >= pairs[1]) {
        return true;
    }
    return false;
}

fn check_pairs_overlap(pairs: &Vec<u32>) -> bool {
    if (pairs[0] <= pairs[2]) & (pairs[2] <= pairs[1]) {
        return true;
    }
    if (pairs[0] <= pairs[3]) & (pairs[3] <= pairs[1]) {
        return true;
    }
    if (pairs[2] <= pairs[0]) & (pairs[0] <= pairs[3]) {
        return true;
    }
    if (pairs[2] <= pairs[1]) & (pairs[1] <= pairs[3]) {
        return true;
    }
    return false;
}

fn solve_part1(fname: &String) -> u32 {
    // Read data file
    let file_content = read_file(&fname);
    let mut n_contained_pairs: u32 = 0;
    for line in file_content.lines() {
        let pairs = parse_line(&line);
        if check_pairs_fully_contained(&pairs) {
            n_contained_pairs += 1;
        }
    }
    n_contained_pairs
}

fn solve_part2(fname: &String) -> u32 {
    // Read data file
    let file_content = read_file(&fname);
    let mut n_overlap_pairs: u32 = 0;
    for line in file_content.lines() {
        let pairs = parse_line(&line);
        if check_pairs_overlap(&pairs) {
            n_overlap_pairs += 1;
        }
    }
    n_overlap_pairs
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
