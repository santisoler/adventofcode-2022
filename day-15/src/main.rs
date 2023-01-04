use std::cmp::max;
use std::io::Read;
use std::time::Instant;
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
        assert!(merged.len() == 1);
        assert_eq!(merged[0][0], 2);
        assert_eq!(merged[0][1], 18);
        let ranges = vec![[2, 10], [11, 14], [16, 18]];
        let merged = merge_ranges(&ranges);
        assert!(merged.len() == 3);
        assert_eq!(merged[0][0], 2);
        assert_eq!(merged[0][1], 10);
        assert_eq!(merged[1][0], 11);
        assert_eq!(merged[1][1], 14);
        assert_eq!(merged[2][0], 16);
        assert_eq!(merged[2][1], 18);
    }

    #[test]
    fn test_rotate() {
        let mut point = Point::from(&[1, 1]);
        point.rotate();
        assert_eq!(point.x, 0);
        assert_eq!(point.y, 2);
        let mut point = Point::from(&[1, 0]);
        point.rotate();
        assert_eq!(point.x, 1);
        assert_eq!(point.y, 1);
        let mut point = Point::from(&[0, 1]);
        point.rotate();
        assert_eq!(point.x, -1);
        assert_eq!(point.y, 1);
        let mut point = Point::from(&[2, 1]);
        point.rotate();
        assert_eq!(point.x, 1);
        assert_eq!(point.y, 3);
    }

    #[test]
    fn test_rotate_rev() {
        let mut point = Point::from(&[2, 1]);
        let expected = point.clone();
        point.rotate();
        point.rotate_reverse();
        assert_eq!(point, expected);
    }

    #[test]
    fn test_part1() {
        let fname = String::from("data/test_input");
        let result = solve_part1(&fname, 10);
        assert_eq!(result, 26);
    }

    #[test]
    fn test_part2() {
        let fname = String::from("data/test_input");
        let result = solve_part2(&fname);
        assert_eq!(result, 56000011);
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

    pub fn rotate(&mut self) {
        // Rotate the point 45 CW degrees and scale to keep them as ints
        let x = self.x.clone();
        let y = self.y.clone();
        self.x = x - y;
        self.y = x + y;
    }

    pub fn rotate_reverse(&mut self) {
        // Rotate the point 45 degrees CCW and scale to keep them as ints
        let x = self.x.clone();
        let y = self.y.clone();
        self.x = (x + y) / 2;
        self.y = (-x + y) / 2;
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Area {
    x1: i64,
    x2: i64,
    y1: i64,
    y2: i64,
}

impl Area {
    pub fn from(x1: &i64, x2: &i64, y1: &i64, y2: &i64) -> Self {
        Area {
            x1: *x1,
            x2: *x2,
            y1: *y1,
            y2: *y2,
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

pub fn solve_part2(fname: &String) -> i64 {
    // Read file content
    let content = read_file(&fname);
    // Parse file
    let mut sensors: Vec<Point> = vec![];
    let mut distances_to_beacons: Vec<i64> = vec![];
    for line in content.lines() {
        let (sensor, beacon) = parse_line(&line);
        let sensor = Point::from(&sensor);
        let beacon = Point::from(&beacon);
        distances_to_beacons.push(get_distance(&sensor, &beacon));
        sensors.push(sensor);
    }
    // Rotate the sensor locations so the areas covered by each one are now squares
    let mut rotated_sensors = sensors.clone();
    for sensor in rotated_sensors.iter_mut() {
        sensor.rotate();
    }
    // Define areas of coverage for each sensor
    let mut areas = Vec::<Area>::new();
    for (sensor, dist) in rotated_sensors.iter().zip(distances_to_beacons.clone()) {
        let x1 = sensor.x - dist;
        let x2 = sensor.x + dist;
        let y1 = sensor.y - dist;
        let y2 = sensor.y + dist;
        areas.push(Area { x1, x2, y1, y2 })
    }
    // Find pairs of area boundaries that leave a space in between
    let mut x_candidates: Vec<i64> = vec![];
    areas.sort_by(|a, b| a.x1.cmp(&b.x1)); // sort areas by x1
    for i in 0..areas.len() - 1 {
        for j in i..areas.len() {
            if areas[i].x2 + 2 == areas[j].x1 {
                x_candidates.push(areas[i].x2 + 1)
            }
        }
    }
    let mut y_candidates: Vec<i64> = vec![];
    areas.sort_by(|a, b| a.y1.cmp(&b.y1)); // sort areas by y1
    for i in 0..areas.len() - 1 {
        for j in i..areas.len() {
            if areas[i].y2 + 2 == areas[j].y1 {
                y_candidates.push(areas[i].y2 + 1)
            }
        }
    }
    // Unrotate the candidates
    let mut points: Vec<Point> = vec![];
    for x in x_candidates.iter() {
        for y in y_candidates.iter() {
            points.push(Point { x: *x, y: *y })
        }
    }
    for point in points.iter_mut() {
        point.rotate_reverse()
    }
    // Remove repeated points
    let mut unique = vec![];
    for point in points.iter() {
        if !unique.contains(point) {
            unique.push(point.clone())
        }
    }
    // Eliminate the points that are already covered by a sensor and keep the only one that could
    // have a beacon
    let mut beacon = Point { x: 0, y: 0 };
    for point in points {
        let mut is_covered = false;
        for (sensor, dist) in sensors.iter().zip(distances_to_beacons.clone()) {
            if get_distance(&sensor, &point) <= dist {
                is_covered = true;
                break;
            }
        }
        if !is_covered {
            beacon = point;
            break;
        }
    }
    // Return the tuning frequency
    beacon.x * 4_000_000 + beacon.y
}

fn main() {
    let fname = String::from("data/input");

    // part 1
    let now = Instant::now();
    let result = solve_part1(&fname, 2000000);
    let elapsed = now.elapsed();
    println!("Solution to part 1: {}", result);
    println!("Elapsed time: {}µs", elapsed.as_micros());

    // part 2
    let now = Instant::now();
    let result = solve_part2(&fname);
    let elapsed = now.elapsed();
    println!("Solution to part 2: {}", result);
    println!("Elapsed time: {}µs", elapsed.as_micros());
}
