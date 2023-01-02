use std::io::Read;
use std::{fs::File, path::Path};

#[cfg(test)]
mod tests {
    use super::*;

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

fn get_extreme_coordinate(sensors: &Vec<Point>) -> (i64, i64) {
    let mut leftmost_distance = i64::MAX;
    let mut rightmost_distance = i64::MIN;
    for sensor in sensors {
        if leftmost_distance > sensor.x {
            leftmost_distance = sensor.x
        }
        if rightmost_distance < sensor.x {
            rightmost_distance = sensor.x
        }
    }
    (leftmost_distance, rightmost_distance)
}

pub fn solve_part1(fname: &String, row: usize) -> i64 {
    let content = read_file(&fname);
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
    let max_distance = distances_to_beacons.iter().max().unwrap();
    let (leftmost_sensor, rightmost_sensor) = get_extreme_coordinate(&sensors);
    let xmin = leftmost_sensor - max_distance;
    let xmax = rightmost_sensor + max_distance;
    let n = (xmax - xmin + 1) as usize;
    let mut count = 0;
    for i in 0..n {
        let point = Point {
            x: xmin + i as i64,
            y: row as i64,
        };
        for j in 0..sensors.len() {
            let sensor = sensors[j];
            let dist_to_beacon = distances_to_beacons[j];
            if get_distance(&sensor, &point) <= dist_to_beacon {
                if !beacons.contains(&point) {
                    count += 1;
                }
                break;
            }
        }
    }
    count
}

fn main() {
    let fname = String::from("data/input");

    // part 1
    let result = solve_part1(&fname, 2000000);
    println!("Solution to part 1: {}", result);
}
