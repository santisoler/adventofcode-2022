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
        assert_eq!(result, 15);
    }

    #[test]
    fn test_part2() {
        let fname = String::from("data/test_input");
        let result = solve_part2(&fname);
        assert_eq!(result, 12);
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

fn solve_part1(fname: &String) -> i32 {
    // Read data file
    let data = read_file(&fname);
    // Compute score
    let mut score: i32 = 0;
    let match_matrix = [[3, 6 ,0], [0, 3, 6], [6, 0 ,3]];
    for line in data.lines() {
        let play = line.split_whitespace().collect::<Vec<&str>>();
        let my_play: usize = match play[1] {
            "X" => 0,
            "Y" => 1,
            "Z" => 2,
            _ => panic!("Found invalid play '{}'", play[1]),
        };
        let opponents_play: usize = match play[0] {
            "A" => 0,
            "B" => 1,
            "C" => 2,
            _ => panic!("Found invalid play '{}'", play[1]),
        };
        // Add to score based on which game I played
        score += my_play as i32 + 1;
        // Add to score based on match results
        score += match_matrix[opponents_play][my_play];
    }
    score
}

fn solve_part2(fname: &String) -> i32 {
    // Read data file
    let data = read_file(&fname);
    // Compute score
    let mut score: i32 = 0;
    for line in data.lines() {
        let play = line.split_whitespace().collect::<Vec<&str>>();
        // Add to score
        match play[1] {
            // I need to loose
            // (no score from the match, just the score from my choice)
            "X" => {
                score += match play[0] {
                    "A" => 3,
                    "B" => 1,
                    "C" => 2,
                    _ => panic!("Found invalid play '{}'", play[0]),
                }
            }
            // I need to draw
            // (3 points from draw score from my choice)
            "Y" => {
                score += match play[0] {
                    "A" => 3 + 1,
                    "B" => 3 + 2,
                    "C" => 3 + 3,
                    _ => panic!("Found invalid play '{}'", play[0]),
                }
            }
            // I need to win
            // (6 points from draw score from my choice)
            "Z" => {
                score += match play[0] {
                    "A" => 6 + 2,
                    "B" => 6 + 3,
                    "C" => 6 + 1,
                    _ => panic!("Found invalid play '{}'", play[0]),
                }
            }
            _ => panic!("Found invalid play '{}'", play[1]),
        }
    }
    score
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
