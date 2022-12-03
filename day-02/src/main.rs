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

    // #[test]
    // fn test_part2() {
    //     let fname = String::from("data/test_input");
    //     let result = solve_part2(&fname);
    //     assert_eq!(result, 45000);
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

fn solve_part1(fname: &String) -> i32 {
    // Read data file
    let data = read_file(&fname);
    // Compute score
    let mut score: i32 = 0;
    for line in data.lines() {
        let play = line.split_whitespace().collect::<Vec<&str>>();
        // Add to score based on which game I played
        score += match play[1] {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => panic!("Found invalid play '{}'", play[1]),
        };
        // Add to score based on how the two players played
        match play[0] {
            "A" => {
                score += match play[1] {
                    "X" => 3,
                    "Y" => 6,
                    "Z" => 0,
                    _ => panic!("Found invalid play '{}'", play[1]),
                }
            }
            "B" => {
                score += match play[1] {
                    "X" => 0,
                    "Y" => 3,
                    "Z" => 6,
                    _ => panic!("Found invalid play '{}'", play[1]),
                }
            }
            "C" => {
                score += match play[1] {
                    "X" => 6,
                    "Y" => 0,
                    "Z" => 3,
                    _ => panic!("Found invalid play '{}'", play[1]),
                }
            }
            _ => panic!("Found invalid play '{}'", play[0]),
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
    // let result = solve_part2(&fname);
    // println!("Solution to part 2: {}", result);
}
