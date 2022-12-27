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
        assert_eq!(result, 13);
    }

    #[test]
    fn test_part2() {
        let fname = String::from("data/test_input");
        let result = solve_part2(&fname);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_part2_larger() {
        let fname = String::from("data/test_input_2");
        let result = solve_part2(&fname);
        assert_eq!(result, 36);
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
    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Err(why) => panic!("couldn't read {}: {}", fname, why),
        Ok(_) => (),
    };
    content
}

fn update_tail_position(head: &[i32; 2], tail: &mut [i32; 2]) -> () {
    // Update the position of the tail based on the position of the head
    if (head[0] - tail[0]).abs() >= 2 && head[1] - tail[1] == 0 {
        // Move it horizontally if needed
        let mut movement = 1;
        if head[0] - tail[0] < 0 {
            movement *= -1
        }
        tail[0] += movement;
    } else if (head[1] - tail[1]).abs() >= 2 && head[0] - tail[0] == 0 {
        // Move it vertically if needed
        let mut movement = 1;
        if head[1] - tail[1] < 0 {
            movement *= -1
        }
        tail[1] += movement;
    } else if (head[0] - tail[0]).abs() >= 2 || (head[1] - tail[1]).abs() >= 2 {
        // Move it diagonally if needed
        let mut movement_x = 1;
        let mut movement_y = 1;
        if head[0] - tail[0] < 0 {
            movement_x *= -1
        }
        if head[1] - tail[1] < 0 {
            movement_y *= -1
        }
        tail[0] += movement_x;
        tail[1] += movement_y;
    }
}

fn solve_part1(fname: &String) -> u32 {
    // Read data file
    let data = read_file(&fname);
    // Define starting positions for the head and the tail
    let mut head: [i32; 2] = [0, 0];
    let mut tail: [i32; 2] = [0, 0];
    // Define a vector with the positions that the tail visited
    // (initialize it with the initial position of tail)
    let mut visited: Vec<[i32; 2]> = vec![tail.clone()];
    // Start reading the movement instructions
    for line in data.lines() {
        let movements: usize = line.split_whitespace().last().unwrap().parse().unwrap();
        let direction = line.split_whitespace().nth(0).unwrap();
        for _ in 0..movements {
            // Update the position of the head
            match direction {
                "R" => head[0] += 1,
                "L" => head[0] -= 1,
                "U" => head[1] += 1,
                "D" => head[1] -= 1,
                _ => panic!("Invalid movement direction '{}'", direction),
            }
            // Update position of the tail
            update_tail_position(&head, &mut tail);
            // Add the new position of the tail to visited if it hasn't been visited
            if !visited.contains(&tail) {
                visited.push(tail.clone())
            }
        }
    }
    visited.len() as u32
}

fn update_knots_positions(knots: &mut Vec<[i32; 2]>) -> () {
    // Update the positions of every knot after the head has been moved
    for i in 1..knots.len() {
        let head = knots[i - 1].clone();
        update_tail_position(&head, &mut knots[i]);
    }
}

fn solve_part2(fname: &String) -> u32 {
    // Read data file
    let data = read_file(&fname);
    // Define starting positions for the head and the multiple knots. The first element is the
    // head, the 10th is the tail.
    let mut knots: Vec<[i32; 2]> = vec![[0, 0]; 10];
    // Define a vector with the positions that the tail (the last knot) visited
    // (initialize it with the initial position of tail)
    let mut visited: Vec<[i32; 2]> = vec![knots.last().unwrap().clone()];
    // Start reading the movement instructions
    for line in data.lines() {
        let movements: usize = line.split_whitespace().last().unwrap().parse().unwrap();
        let direction = line.split_whitespace().nth(0).unwrap();
        for _ in 0..movements {
            // Update the position of the head
            match direction {
                "R" => knots[0][0] += 1,
                "L" => knots[0][0] -= 1,
                "U" => knots[0][1] += 1,
                "D" => knots[0][1] -= 1,
                _ => panic!("Invalid movement direction '{}'", direction),
            }
            // Update position of the knots
            update_knots_positions(&mut knots);
            // Add the new position of the tail to visited if it hasn't been visited
            let tail_position = knots.last().unwrap();
            if !visited.contains(&tail_position) {
                visited.push(tail_position.clone())
            }
        }
    }
    visited.len() as u32
}

fn main() {
    let fname = String::from("data/input");

    // part 1
    let result = solve_part1(&fname);
    println!("Solution to part 1: {}", result);

    // // part 2
    let result = solve_part2(&fname);
    println!("Solution to part 2: {}", result);
}
