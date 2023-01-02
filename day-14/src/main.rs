use std::collections::HashMap;
use std::io::Read;
use std::{fs::File, path::Path};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_vertical_line() {
        let mut sub = Subsurface::new();
        sub.add_vertical_line(&500, &1, &3);
        sub.add_vertical_line(&500, &5, &10);
        let expected_grid = vec![
            false, true, true, true, false, true, true, true, true, true, true,
        ];
        assert_eq!(*sub.0.get(&500).unwrap(), expected_grid);
    }

    #[test]
    fn test_add_horizontal_line() {
        let mut sub = Subsurface::new();
        sub.add_horizontal_line(&2, &499, &501);
        let expected_grid = vec![false, false, true];
        for x in [499, 500, 501].iter() {
            assert_eq!(*sub.0.get(&x).unwrap(), expected_grid);
        }
    }

    #[test]
    fn test_part1() {
        let fname = String::from("data/test_input");
        let result = solve_part1(&fname);
        assert_eq!(result, 24);
    }

    #[test]
    fn test_part2() {
        let fname = String::from("data/test_input");
        let result = solve_part2(&fname);
        assert_eq!(result, 93);
    }
}

#[derive(Debug)]
struct Subsurface(HashMap<u64, Vec<bool>>);

impl Subsurface {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn add_rock_segment(&mut self, start: &[u64; 2], end: &[u64; 2]) {
        if start[0] == end[0] {
            self.add_vertical_line(&start[0], &start[1], &end[1])
        }
        if start[1] == end[1] {
            self.add_horizontal_line(&start[1], &start[0], &end[0])
        }
    }

    pub fn add_sand_grain(&mut self, grain: &SandGrain) {
        // Adds a resting sand grain to the subsurface
        let x = grain.0[0];
        let y = grain.0[1];
        match self.0.get_mut(&x) {
            Some(column) => {
                if column.len() < y as usize + 1 {
                    column.extend(vec![false; y as usize + 1 - column.len()]);
                }
                column[y as usize] = true;
            }
            None => {
                let mut column = vec![false; y as usize + 1];
                column[y as usize] = true;
                self.0.insert(x.clone(), column);
            }
        }
    }

    pub fn is_blocked(&self, position: &[u64; 2]) -> bool {
        // Return true if the given position is currently blocked by a rock unit or a sand grain
        let x = position[0];
        let y = position[1];
        match self.0.get(&x) {
            Some(column) => {
                if column.len() < y as usize + 1 {
                    false // return false if there is no strucure below that point for this column
                } else {
                    column[y as usize] // return the bool stored in that position of the grid
                }
            }
            None => false, // return false if the column isn't registered yet in the subsurface
        }
    }

    pub fn get_deepest_level(&self) -> u64 {
        // Return the maximum depth of the current subsurface
        let mut deepest_level = 0;
        for (_, column) in self.0.iter() {
            if column.len() > deepest_level + 1 {
                deepest_level = column.len() - 1
            }
        }
        deepest_level as u64
    }

    fn add_vertical_line(&mut self, x: &u64, y1: &u64, y2: &u64) {
        // Add a single vertical line segment to the subsurface
        let (ymin, ymax) = match y1 < y2 {
            true => (*y1, *y2),
            false => (*y2, *y1),
        };
        let column = self.0.entry(x.clone()).or_insert(vec![]);
        if column.len() < ymax as usize + 1 {
            column.extend(vec![false; ymax as usize + 1 - column.len()])
        }
        for depth in 0..column.len() {
            if ymin <= depth as u64 && depth as u64 <= ymax {
                column[depth] = true;
            }
        }
    }

    fn add_horizontal_line(&mut self, y: &u64, x1: &u64, x2: &u64) {
        // Add a single horizontal line segment to the subsurface
        let (xmin, xmax) = match x1 < x2 {
            true => (*x1, *x2),
            false => (*x2, *x1),
        };
        for x in xmin..xmax + 1 {
            let column = self.0.entry(x.clone()).or_insert(vec![]);
            if column.len() < *y as usize + 1 {
                column.extend(vec![false; *y as usize + 1 - column.len()])
            }
            column[*y as usize] = true;
        }
    }
}

struct SandGrain([u64; 2]);
//
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
            subsurface.add_rock_segment(&points[i], &points[i + 1])
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

pub fn solve_part2(fname: &String) -> u64 {
    // Parse file and create subsurface structure
    let mut subsurface = parse_file(&fname);
    // Get the deepest level of the subsurface structure
    let deepest_level = subsurface.get_deepest_level();
    // Define the depth to the floor
    let floor_depth = deepest_level + 2;
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
            // Check if the sand grain hasn't moved or if it hit the position right above the
            // floor. In any one of these two cases, put the sand grain to rest.
            if !has_moved || sand_grain.0[1] == floor_depth - 1 {
                subsurface.add_sand_grain(&sand_grain);
                resting_sand_grains += 1;
                // Check if it's resting in the pouring point
                if sand_grain.0 == pouring_point {
                    return resting_sand_grains;
                }
                break;
            }
        }
    }
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
