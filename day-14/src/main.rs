use std::io::Read;
use std::{fs::File, path::Path};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let fname = String::from("data/test_input");
        let result = solve_part1(&fname);
        assert_eq!(result, 24);
    }
}

#[derive(Debug)]
struct Subsurface(Vec<[u64; 2]>);

impl Subsurface {
    fn new() -> Self {
        Self(vec![])
    }

    fn add_rock_line(&mut self, start: &[u64; 2], end: &[u64; 2]) {
        if start[0] == end[0] {
            let mut bounds = [start[1], end[1]];
            bounds.sort();
            for i in bounds[0]..bounds[1] + 1 {
                self.0.push([start[0], i])
            }
        } else if start[1] == end[1] {
            let mut bounds = [start[0], end[0]];
            bounds.sort();
            for i in bounds[0]..bounds[1] + 1 {
                self.0.push([i, start[1]])
            }
        }
    }

    fn add_sand_grain(&mut self, grain: &SandGrain) {
        // Adds a restingsand grain to the subsurface
        self.0.push(grain.0.clone())
    }

    fn is_blocked(&self, position: &[u64; 2]) -> bool {
        // Return true if the given position is currently blocked by a rock unit or a sand grain
        self.0.contains(position)
    }

    fn get_deepest_level(&self) -> u64 {
        // Return the maximum depth of the current subsurface
        let y: Vec<u64> = self.0.iter().map(|x| x[1]).collect();
        *y.iter().max().unwrap()
    }
}

struct SandGrain([u64; 2]);

impl SandGrain {
    fn try_move(&mut self, subsurface: &Subsurface) -> bool {
        // Move the sandgrain one time interval. Return true if it has moved, return false if not.
        let mut next_position = [self.0[0], self.0[1] + 1];
        if !subsurface.is_blocked(&next_position) {
            self.0 = next_position;
            return true;
        }
        next_position = [self.0[0] - 1, self.0[1] + 1];
        if !subsurface.is_blocked(&next_position) {
            self.0 = next_position;
            return true;
        }
        next_position = [self.0[0] + 1, self.0[1] + 1];
        if !subsurface.is_blocked(&next_position) {
            self.0 = next_position;
            return true;
        }
        false
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

fn parse_file(fname: &String) -> Subsurface {
    // Parse input file
    let content = read_file(&fname);
    let mut subsurface = Subsurface::new();
    for line in content.lines() {
        let mut points = Vec::<[u64; 2]>::new();
        for string in line.split_whitespace().filter(|x| *x != "->") {
            let point: Vec<u64> = string.split(",").map(|x| x.parse().unwrap()).collect();
            points.push([point[0], point[1]]);
        }
        for i in 0..points.len() - 1 {
            subsurface.add_rock_line(&points[i], &points[i + 1])
        }
    }
    subsurface
}

pub fn solve_part1(fname: &String) -> u64 {
    // Parse file and create subsurface structure
    let mut subsurface = parse_file(&fname);
    // Get the deepest level of the subsurface structure
    let deepest_level = subsurface.get_deepest_level();
    // Define default pouring point
    let pouring_point = [500, 0];
    // Initialize counter for number of resting sand grains
    let mut resting_sand_grains: u64 = 0;
    // Start dropping sand grains
    loop {
        let mut sand_grain = SandGrain(pouring_point.clone());
        loop {
            // Try to move the sand grain (it moves one step if it's possible, otherwise it stays
            // there and returns false)
            let has_moved = sand_grain.try_move(&subsurface);
            // Check if the sand grain is now resting
            if !has_moved {
                subsurface.add_sand_grain(&sand_grain);
                resting_sand_grains += 1;
                break;
            }
            // Check if the sand grain is falling into the abyss
            if sand_grain.0[1] > deepest_level {
                return resting_sand_grains;
            }
        }
    }
}

fn main() {
    let fname = String::from("data/input");

    // part 1
    let result = solve_part1(&fname);
    println!("Solution to part 1: {}", result);
}
