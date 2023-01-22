use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::time::Instant;

const COLUMN_WIDTH: usize = 7;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let fname = String::from("data/test_input");
        assert_eq!(solve_part1(&fname), 3068);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Block {
    x: u64,
    y: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Rock {
    x: u64,             // x coordinate of the lower left block
    y: u64,             // y coordinate of the lower left block
    blocks: Vec<Block>, // list of blocks that form the rock as offsets from x and y
}

#[derive(Debug)]
struct Chamber {
    map: Vec<[bool; COLUMN_WIDTH]>,
}

#[derive(Debug, Clone, Copy)]
enum Push {
    Left,
    Right,
}

impl Rock {
    fn fall(&mut self) {
        // Fall rock a single unit
        if self.y > 0 {
            self.y -= 1;
        }
    }

    fn push(&mut self, direction: &Push) {
        // Push rock in the given direction
        match direction {
            Push::Left => {
                if self.x > 0 {
                    self.x -= 1;
                }
            }
            Push::Right => {
                let max_x = self.get_max_x() as usize;
                if max_x < COLUMN_WIDTH - 1 {
                    self.x += 1
                }
            }
        };
    }

    fn get_max_x(&self) -> u64 {
        // Return the maximum x of the rock blocs
        self.x + self.blocks.iter().map(|b| b.x).max().unwrap()
    }

    fn get_max_y(&self) -> u64 {
        // Return the maximum y of the rock blocs
        self.y + self.blocks.iter().map(|b| b.y).max().unwrap()
    }

    fn new_hbar(heighest_position: &u64) -> Self {
        Self {
            x: 2,
            y: heighest_position + 3,
            blocks: vec![
                Block { x: 0, y: 0 },
                Block { x: 1, y: 0 },
                Block { x: 2, y: 0 },
                Block { x: 3, y: 0 },
            ],
        }
    }

    fn new_vbar(heighest_position: &u64) -> Self {
        Self {
            x: 2,
            y: heighest_position + 3,
            blocks: vec![
                Block { x: 0, y: 0 },
                Block { x: 0, y: 1 },
                Block { x: 0, y: 2 },
                Block { x: 0, y: 3 },
            ],
        }
    }

    fn new_cross(heighest_position: &u64) -> Self {
        Self {
            x: 2,
            y: heighest_position + 3,
            blocks: vec![
                Block { x: 1, y: 0 },
                Block { x: 1, y: 1 },
                Block { x: 1, y: 2 },
                Block { x: 0, y: 1 },
                Block { x: 2, y: 1 },
            ],
        }
    }

    fn new_lshape(heighest_position: &u64) -> Self {
        Self {
            x: 2,
            y: heighest_position + 3,
            blocks: vec![
                Block { x: 0, y: 0 },
                Block { x: 1, y: 0 },
                Block { x: 2, y: 0 },
                Block { x: 2, y: 0 },
                Block { x: 2, y: 1 },
                Block { x: 2, y: 2 },
            ],
        }
    }

    fn new_square(heighest_position: &u64) -> Self {
        Self {
            x: 2,
            y: heighest_position + 3,
            blocks: vec![
                Block { x: 0, y: 0 },
                Block { x: 0, y: 1 },
                Block { x: 1, y: 0 },
                Block { x: 1, y: 1 },
            ],
        }
    }
}
impl Chamber {
    fn new() -> Self {
        Self { map: vec![] }
    }

    fn get_highest_position(&self) -> u64 {
        self.map.len() as u64
    }

    fn can_hold_rock(&self, rock: &Rock) -> bool {
        // Determine if the chamber can hold the given rock
        for block in rock.blocks.iter() {
            let x = (rock.x + block.x) as usize;
            let y = (rock.y + block.y) as usize;
            if y >= self.map.len() {
                continue;
            }
            if self.map[y][x] {
                return false;
            }
        }
        true
    }

    fn add_rock(&mut self, rock: &Rock) {
        // Add a rock to the chamber map
        let max_height_of_rock = rock.get_max_y();
        if (max_height_of_rock + 1) as usize > self.map.len() {
            let new_rows = (max_height_of_rock + 1) as usize - self.map.len();
            for _ in 0..new_rows {
                self.map.push([false; COLUMN_WIDTH]);
            }
        }
        for block in rock.blocks.iter() {
            let x = (rock.x + block.x) as usize;
            let y = (rock.y + block.y) as usize;
            self.map[y][x] = true;
        }
    }
}

impl Display for Chamber {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let print: Vec<String> = self
            .map
            .iter()
            .rev()
            .map(|row| row.iter().map(|v| if *v { "#" } else { "." }).collect())
            .collect();
        for row in print.iter() {
            writeln!(f, "|{}|", row);
        }
        writeln!(f, "+{}+", "-".repeat(COLUMN_WIDTH))
    }
}

fn read_file(fname: &String) -> String {
    // Open file
    let path = Path::new(&fname);
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", fname, why),
        Ok(file) => file,
    };
    // Read file
    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Err(why) => panic!("couldn't read {}: {}", fname, why),
        Ok(_) => (),
    };
    content
}

fn parse_jet_pushes(fname: &String) -> Vec<Push> {
    let content = read_file(&fname);
    let jet_pushes = content
        .lines()
        .nth(0)
        .unwrap()
        .chars()
        .map(|c| match c {
            '>' => Push::Right,
            '<' => Push::Left,
            _ => panic!("bla"),
        })
        .collect();
    jet_pushes
}

fn solve_part1(fname: &String) -> u64 {
    let jet_pushes = parse_jet_pushes(&fname);
    let mut chamber = Chamber::new();
    let mut push_i = 0;
    for n_rock in 0..2022 {
        // Create new rock
        let highest_position = chamber.get_highest_position();
        let mut rock = match n_rock % 5 {
            0 => Rock::new_hbar(&highest_position),
            1 => Rock::new_cross(&highest_position),
            2 => Rock::new_lshape(&highest_position),
            3 => Rock::new_vbar(&highest_position),
            4 => Rock::new_square(&highest_position),
            _ => panic!("Not found '{}'", n_rock),
        };
        // Loop until the rock rests
        loop {
            // Get current push
            let push = jet_pushes[push_i % jet_pushes.len()];
            push_i += 1;
            // Push the rock
            let mut test_rock = rock.clone();
            test_rock.push(&push);
            if chamber.can_hold_rock(&test_rock) {
                rock = test_rock;
            }
            // Make the rock fall
            let mut test_rock = rock.clone();
            test_rock.fall();
            if test_rock.y == rock.y || !chamber.can_hold_rock(&test_rock) {
                chamber.add_rock(&rock);
                break;
            }
            rock = test_rock;
        }
    }
    chamber.get_highest_position()
}

fn main() {
    let fname = String::from("data/input");

    // part 1
    let now = Instant::now();
    let result = solve_part1(&fname);
    let elapsed = now.elapsed();
    println!("Solution to part 1: {}", result);
    println!("Elapsed time: {}µs", elapsed.as_millis());
}
