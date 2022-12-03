use std::fs::File;
use std::io::Read;
use std::path::Path;

// Define a matrix that holds the match scores.
// Each row correspond to the opponent's play. Each column corresponds to my
// play. E.g.: if the opponent plays paper (row 1) and I play scissors (col 2)
// I win and get 6 points.
const MATCH_SCORES: [[i32; 3]; 3] = [[3, 6, 0], [0, 3, 6], [6, 0, 3]];

// Define a matrix that holds my play scores based on expected outcomes.
// Each row correspond to the opponent's play. Each column corresponds to the
// expected outcome:
// loose, draw and win.
// play. E.g.: if the opponent plays paper (row 1) and I need to win (col 2)
// I need to play scissors, obtaining a score of 3 for my play.
const PLAY_SCORES: [[i32; 3]; 3] = [[3, 1, 2], [1, 2, 3], [2, 3, 1]];

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
    for line in data.lines() {
        let play = line.split_whitespace().collect::<Vec<&str>>();
        // Transform plays in column and rows indices for the MATCH_MATRIX
        // (rows are for opponent's play, columns for my play)
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
        score += MATCH_SCORES[opponents_play][my_play];
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
        // Transform opponent's play in rows indices for the PLAY_SCORES matrix
        let opponents_play: usize = match play[0] {
            "A" => 0,
            "B" => 1,
            "C" => 2,
            _ => panic!("Found invalid play '{}'", play[0]),
        };
        // Transform expected results in rows indices for the PLAY_SCORES matrix
        let expected_result: usize = match play[1] {
            "X" => 0,
            "Y" => 1,
            "Z" => 2,
            _ => panic!("Found invalid play '{}'", play[1]),
        };
        // Add to score based on which game I played
        score += PLAY_SCORES[opponents_play][expected_result];
        // Add to score based on match results
        score += expected_result as i32 * 3;
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
