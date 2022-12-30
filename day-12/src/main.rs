use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_neighbours() {
        let fname = String::from("data/test_input");
        let (topo, start, end) = parse_file(&fname);
        let point = Point {
            steps: 0,
            row: start[0],
            col: start[1],
        };
        let expected = vec![[1, 0], [0, 1]];
        assert_eq!(point.get_neighbours(&topo), expected);
        let point = Point {
            steps: 0,
            row: end[0],
            col: end[1],
        };
        let expected = vec![[3, 5], [1, 5], [2, 6], [2, 4]];
        assert_eq!(point.get_neighbours(&topo), expected);
        let point = Point {
            steps: 0,
            row: 4,
            col: 7,
        };
        let expected = vec![[3, 7], [4, 6]];
        assert_eq!(point.get_neighbours(&topo), expected);
        let point = Point {
            steps: 0,
            row: 3,
            col: 2,
        };
        let expected = vec![[4, 2], [2, 2], [3, 1]];
        assert_eq!(point.get_neighbours(&topo), expected);
    }

    #[test]
    fn test_part1() {
        let fname = String::from("data/test_input");
        let result = solve_part1(&fname);
        assert_eq!(result, 31);
    }

    // #[test]
    // fn test_part2() {
    //     let fname = String::from("data/test_input");
    //     let result = solve_part2(&fname);
    //     assert_eq!(result, 2713310158);
    // }
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

pub fn parse_file(fname: &String) -> (Vec<Vec<u64>>, [usize; 2], [usize; 2]) {
    // Return topo map with locations of the start and end point
    let content = read_file(&fname);
    let mut topo = vec![];
    let mut start = [0, 0];
    let mut end = [0, 0];
    for (i, line) in content.lines().enumerate() {
        let mut row = vec![];
        for (j, c) in line.chars().enumerate() {
            match c {
                'S' => {
                    start = [i, j];
                    row.push(0);
                }
                'E' => {
                    end = [i, j];
                    row.push(25);
                }
                _ => row.push(c as u64 - 97),
            };
        }
        topo.push(row);
    }
    (topo, start, end)
}

#[derive(Debug, Eq, PartialEq)]
pub struct Point {
    steps: u64,
    row: usize,
    col: usize,
}

impl Point {
    fn get_neighbours(&self, topo: &Vec<Vec<u64>>) -> Vec<[usize; 2]> {
        // Return the neighbours of the point.
        //
        // Only points that are accesible from the point are considered neighbours. So, if
        // a adjacent point has a height difference greater than 1, then this is not a neighbour.
        let mut neighbours = vec![];
        let height = topo[self.row][self.col];
        let indices = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        for (i, j) in indices.iter() {
            if self.row == 0 && *i < 0 {
                continue;
            }
            if self.row == topo.len() - 1 && *i > 0 {
                continue;
            }
            if self.col == 0 && *j < 0 {
                continue;
            }
            if self.col == topo[0].len() - 1 && *j > 0 {
                continue;
            }
            let row = (self.row as i64 + *i) as usize;
            let col = (self.col as i64 + *j) as usize;
            if height + 1 >= topo[row][col] {
                neighbours.push([row, col]);
            }
        }
        neighbours
    }
}

impl PartialOrd for Point {
    // Implement PartialOrd for Point
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point {
    // Implement Ord in reverse way, so the priority queue of BinaryHeap becomes a min-heap
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.steps.cmp(&self.steps)
    }
}

fn solve_part1(fname: &String) -> u64 {
    // Parse file
    let (topo, start, end) = parse_file(&fname);
    let ncols = topo[0].len();
    let nrows = topo.len();

    // Create a binary heap
    let mut heap = BinaryHeap::new();

    // Define 2d vec for flagging points as visited
    let mut visited: Vec<Vec<bool>> = vec![vec![false; ncols]; nrows];

    // Define 2d vec for keeping track of min number of steps needed to arrive to each point
    let mut steps: Vec<Vec<u64>> = vec![vec![u64::MAX; ncols]; nrows];

    // Add starting point to the BinaryHeap with zero steps
    heap.push(Point {
        steps: 0,
        row: start[0],
        col: start[1],
    });
    steps[start[0]][start[1]] = 0;

    // Start Dijsktra algorithm
    while !heap.is_empty() {
        // Pop the point in the heap with the smallest number of steps
        let point = heap.pop().unwrap();
        // End the loop if we have arrived to the goal
        if point.row == end[0] && point.col == end[1] {
            return point.steps;
        }
        // Mark the current point as visited
        visited[point.row][point.col] = true;
        // Start checking the neighbours
        for [row, col] in point.get_neighbours(&topo) {
            // Ignore visited neighbours
            if visited[row][col] {
                continue;
            };
            // Push neighbours to the heap
            if point.steps + 1 < steps[row][col] {
                heap.push(Point {
                    steps: point.steps + 1,
                    row,
                    col,
                });
                steps[row][col] = point.steps + 1;
            }
        }
    }
    println!("Couldn't reach the goal");
    0
}

fn main() {
    let fname = String::from("data/input");
    // let fname = String::from("data/test_input");

    // part 1
    let result = solve_part1(&fname);
    println!("Solution to part 1: {}", result);

    // part 2
    // let result = solve_part2(&fname);
    // println!("Solution to part 2: {}", result);
}
