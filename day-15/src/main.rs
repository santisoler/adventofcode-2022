use std::cmp::max;
use std::io::Read;
use std::{fs::File, path::Path};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge() {
        let a = [2, 10];
        let b = [2, 10];
        assert_eq!(merge(&a, &b).unwrap(), [2, 10]);
        let a = [2, 10];
        let b = [8, 14];
        assert_eq!(merge(&a, &b).unwrap(), [2, 14]);
        let a = [-2, 14];
        let b = [2, 10];
        assert_eq!(merge(&a, &b).unwrap(), [-2, 14]);
        let a = [-2, 8];
        let b = [2, 10];
        assert_eq!(merge(&a, &b).unwrap(), [-2, 10]);
        let a = [-2, 2];
        let b = [2, 14];
        assert_eq!(merge(&a, &b).unwrap(), [-2, 14]);
    }

    #[test]
    fn test_merge_ranges() {
        let ranges = vec![[2, 10], [2, 14], [14, 18]];
        let merged = merge_ranges(&ranges);
        println!("{:?}", merged);
        assert!(merged.len() == 1);
        assert_eq!(merged[0][0], 2);
        assert_eq!(merged[0][1], 18);
        let ranges = vec![[2, 10], [11, 14], [16, 18]];
        let merged = merge_ranges(&ranges);
        println!("{:?}", merged);
        assert!(merged.len() == 3);
        assert_eq!(merged[0][0], 2);
        assert_eq!(merged[0][1], 10);
        assert_eq!(merged[1][0], 11);
        assert_eq!(merged[1][1], 14);
        assert_eq!(merged[2][0], 16);
        assert_eq!(merged[2][1], 18);
    }

    #[test]
    fn test_part1() {
        let fname = String::from("data/test_input");
        let result = solve_part1(&fname, 10);
        assert_eq!(result, 26);
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

#[derive(Debug, Clone, Copy)]
pub struct Point {
    x: i64,
    y: i64,
}

impl Point {
    pub fn from(position: &[i64; 2]) -> Self {
        Point {
            x: position[0],
            y: position[1],
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

fn get_distance(point_a: &Point, point_b: &Point) -> i64 {
    // Compute Manhattan distance between two points
    (point_a.x - point_b.x).abs() + (point_a.y - point_b.y).abs()
}

fn get_range_of_coverage(
    sensor: &Point,
    distance_closes_beacon: &i64,
    row: &i64,
) -> Option<[i64; 2]> {
    // Return the range of x coordinates that the current sensor covers in the given row
    //
    // Return None if the coverage of the sensor doesn't intersect that row
    let delta_x = distance_closes_beacon - (sensor.y - row).abs();
    if delta_x <= 0 {
        return None;
    };
    Some([sensor.x - delta_x, sensor.x + delta_x])
}

fn merge_ranges(ranges: &Vec<[i64; 2]>) -> Vec<[i64; 2]> {
    // Sort ranges by the first coordinate
    let mut ranges = ranges.clone();
    ranges.sort_by(|a, b| a[0].cmp(&b[0]));
    let mut merged = vec![ranges[0].clone()];
    for i in 1..ranges.len() {
        let r = merged.pop().unwrap();
        if let Some(union) = merge(&r, &ranges[i]) {
            merged.push(union);
        } else {
            merged.push(r);
            merged.push(ranges[i].clone())
        }
    }
    merged.sort_by(|a, b| a[0].cmp(&b[0]));
    merged
}

fn merge(a: &[i64; 2], b: &[i64; 2]) -> Option<[i64; 2]> {
    // Compute the union of two sorted ranges
    //
    // Double check if the ranges are sorted
    assert!(a[0] <= b[0]);
    // Check if the two ranges overlap
    if a[1] < b[0] {
        return None; // the ranges are not overlapping, union is the null set
    }
    // If so, compute the end of the union as the max end of first and second
    Some([a[0], max(a[1], b[1])])
}

fn parse_line(line: &str) -> ([i64; 2], [i64; 2]) {
    let l: Vec<i64> = line
        .split_whitespace()
        .enumerate()
        .filter(|(i, _)| [2, 3, 8, 9].contains(i))
        .map(|(_, word)| word)
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| {
            x.replace("x", "")
                .replace("y", "")
                .replace("=", "")
                .replace(",", "")
                .replace(":", "")
                .parse::<i64>()
                .unwrap()
        })
        .collect();
    ([l[0], l[1]], [l[2], l[3]])
}

pub fn solve_part1(fname: &String, row: i64) -> i64 {
    // Read file content
    let content = read_file(&fname);
    // Parse file
    let mut sensors: Vec<Point> = vec![];
    let mut beacons: Vec<Point> = vec![];
    let mut distances_to_beacons: Vec<i64> = vec![];
    for line in content.lines() {
        let (sensor, beacon) = parse_line(&line);
        let sensor = Point::from(&sensor);
        let beacon = Point::from(&beacon);
        distances_to_beacons.push(get_distance(&sensor, &beacon));
        sensors.push(sensor);
        if !beacons.contains(&beacon) {
            beacons.push(beacon)
        }
    }
    // Get the ranges of x coordinate in the given row where no beacon could be
    let mut ranges: Vec<[i64; 2]> = vec![];
    for (sensor, distance) in sensors.iter().zip(distances_to_beacons) {
        let result = get_range_of_coverage(&sensor, &distance, &row);
        match result {
            Some(range) => ranges.push(range.clone()),
            None => continue,
        }
    }
    // Get unique ranges
    let ranges = merge_ranges(&ranges);
    // Count how many positions cannot contain a beacon
    let beacons_inrow = beacons
        .iter()
        .filter(|b| b.y == row)
        .collect::<Vec<&Point>>();
    let mut n_positions = 0;
    for range in ranges.iter() {
        // Add elements within the range
        n_positions += range[1] - range[0] + 1;
        // Don't count positions where we already have a beacon
        for beacon in beacons_inrow.iter() {
            if range[0] <= beacon.x && beacon.x <= range[1] {
                n_positions -= 1;
            }
        }
    }
    n_positions
}

fn main() {
    let fname = String::from("data/input");

    // part 1
    let result = solve_part1(&fname, 2000000);
    println!("Solution to part 1: {}", result);
}
