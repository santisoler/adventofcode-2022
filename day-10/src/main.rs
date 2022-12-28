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
        assert_eq!(result, 13140);
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

fn update_signal_strenght(cycle: &u64, x: &i64, signal_strength: &mut i64) {
    if (*cycle as i64 - 20) % 40 == 0 {
        *signal_strength += *cycle as i64 * x;
    }
}

fn solve_part1(fname: &String) -> i64 {
    // Read data file
    let data = read_file(&fname);
    // Initialize variables
    let mut cycle: u64 = 0;
    let mut x: i64 = 1;
    let mut signal_strength: i64 = 0;
    // Read instructions
    for line in data.lines() {
        let instruction = line.split_whitespace().nth(0).unwrap();
        match instruction {
            "noop" => {
                cycle += 1;
                update_signal_strenght(&cycle, &x, &mut signal_strength);
            }
            "addx" => {
                for i in 0..2 {
                    // End the current cycle
                    cycle += 1;
                    update_signal_strenght(&cycle, &x, &mut signal_strength);
                    // Add value to X only AFTER the second cycle of addx
                    if i == 1 {
                        let value: i64 = line.split_whitespace().last().unwrap().parse().unwrap();
                        x += value;
                    }
                }
            }
            _ => panic!("Unknown instruction '{}'", instruction),
        }
    }
    signal_strength
}

fn draw_pixel_on_crt(crt: &mut [char; 40 * 6], cycle: &u64, x: &i64) {
    // Draw pixel at the given position based on the location of x
    //
    // The position of the pixel based on the cycle
    // Get the horizontal position of the current pixel
    let horizontal_position: i64 = *cycle as i64 % 40;
    if (x - horizontal_position).abs() <= 1 {
        crt[*cycle as usize] = '#';
    }
}

fn solve_part2(fname: &String) -> [char; 40 * 6] {
    // Read data file
    let data = read_file(&fname);
    // Initialize variables
    let mut cycle: u64 = 0;
    let mut x: i64 = 1;
    let mut crt: [char; 40 * 6] = ['.'; 40 * 6];
    // Read instructions
    for line in data.lines() {
        let instruction = line.split_whitespace().nth(0).unwrap();
        match instruction {
            "noop" => {
                // Start current cycle and draw pixel in crt.
                draw_pixel_on_crt(&mut crt, &cycle, &x);
                // End the current cycle
                cycle += 1;
            }
            "addx" => {
                for i in 0..2 {
                    // Start current cycle and draw pixel in crt.
                    draw_pixel_on_crt(&mut crt, &cycle, &x);
                    // End the current cycle
                    cycle += 1;
                    // Add value to X only AFTER the second cycle of addx
                    if i == 1 {
                        let value: i64 = line.split_whitespace().last().unwrap().parse().unwrap();
                        x += value;
                    }
                }
            }
            _ => panic!("Unknown instruction '{}'", instruction),
        }
    }
    crt
}

fn main() {
    let fname = String::from("data/input");

    // part 1
    let result = solve_part1(&fname);
    println!("Solution to part 1: {}", result);

    // // part 2
    let result = solve_part2(&fname);
    println!("Solution to part 2:");
    for i in 0..6 {
        let crt: String = result[40 * i..40 * (i + 1)].iter().cloned().collect();
        println!("{}", crt);
    }
}
