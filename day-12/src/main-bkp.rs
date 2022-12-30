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
        let expected = vec![[2, 4]];
        assert_eq!(point.get_neighbours(&topo), expected);
        let point = Point {
            steps: 0,
            row: 4,
            col: 7,
        };
        let expected = vec![[3, 7], [4, 6]];
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
                    row.push(26);
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
        if self.row != 0 {
            let indices = [self.row - 1, self.col];
            if height + 1 >= topo[indices[0]][indices[1]] {
                neighbours.push(indices)
            }
        }
        if self.row != topo.len() - 1 {
            let indices = [self.row + 1, self.col];
            if height + 1 >= topo[indices[0]][indices[1]] {
                neighbours.push(indices)
            }
        }
        if self.col != 0 {
            let indices = [self.row, self.col - 1];
            if height + 1 >= topo[indices[0]][indices[1]] {
                neighbours.push(indices)
            }
        }
        if self.col != topo[0].len() - 1 {
            let indices = [self.row, self.col + 1];
            if height + 1 >= topo[indices[0]][indices[1]] {
                neighbours.push(indices)
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

    // Get number of rows and columns in the topo map
    let nrows = topo.len();
    let ncols = topo[0].len();

    // Create a 2d vec for storing the visited state of each point
    let mut visited: Vec<Vec<bool>> = vec![vec![false; ncols]; nrows];

    // Create a 2d vec for storing the steps for each point (initialize to infinity)
    let mut steps: Vec<Vec<u64>> = vec![vec![u64::MAX; ncols]; nrows];

    // Create a binary heap
    let mut heap = BinaryHeap::new();

    // Add starting point to the BinaryHeap with zero steps
    heap.push(Point {
        steps: 0,
        row: start[0],
        col: start[1],
    });
    steps[start[0]][start[1]] = 0;

    // Start Dijsktra algorithm
    while !heap.is_empty() {
        // for _ in 0..40 {
        // Pop the point in the heap with the smallest number of steps
        let point = heap.pop().unwrap();
        // End the loop if we have arrived to the goal
        if point.row == end[0] && point.col == end[1] {
            println!("Found end!");
            return point.steps;
        }
        // Mark the current point as visited
        visited[point.row][point.col] = true;
        // Find unvisited neighbours of the current point
        let neighbours = point.get_neighbours(&topo);
        let unvisited_neighbours: Vec<[usize; 2]> = neighbours
            .iter()
            .filter(|x| !visited[x[0]][x[1]])
            .cloned()
            .collect();
        // Start checking the neighbours
        for [row, col] in unvisited_neighbours {
            // // Add neighbours to the heap only if the number of steps from the current point is
            // // lower than the number of steps already calculated for each neighbour
            // if point.steps + 1 < steps[row][col] {
            heap.push(Point {
                steps: point.steps + 1,
                row,
                col,
            });
            steps[row][col] = point.steps + 1;
            // }
        }
    }
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
