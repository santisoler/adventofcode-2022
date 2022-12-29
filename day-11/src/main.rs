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
        assert_eq!(result, 10605);
    }

    #[test]
    fn test_part2() {
        let fname = String::from("data/test_input");
        let result = solve_part2(&fname);
        assert_eq!(result, 2713310158);
    }
}

#[derive(Debug)]
pub struct Monkey {
    pub items_worry: Vec<u64>, // stack of items, each element is its worry level
    pub operator: String,      // operation that should be carried out ("+", "-", "*" "**")
    pub operation_value: u64,  // value associated with the operation. For old set it to 0.
    pub divisible_by: u64,     // test if the worry level is divisible by this value
    pub throw_to_if_pass: usize, // if test pass, throw the item the monkey given by this value
    pub throw_to_if_fail: usize, // if test fail, throw the item to monkey given by this value
    pub inspection_counter: u64, // counter for the inspections carried out by the monkey
}

impl Monkey {
    pub fn inspect(&mut self) {
        // Inspect the objects that the monkey is holding
        //
        // Modify the worry level of each of the items that the monkey is holding.
        // This method divides the worry level by 3 after each operation.
        self.items_worry.iter_mut().for_each(|item_worry| {
            *item_worry = match self.operator.as_str() {
                "+" => *item_worry + self.operation_value,
                "-" => *item_worry - self.operation_value,
                "*" => *item_worry * self.operation_value,
                "**" => *item_worry * *item_worry,
                _ => panic!("Invalid operation '{}'", self.operator),
            };
            *item_worry /= 3;
            self.inspection_counter += 1;
        });
    }

    pub fn inspect_with_modulo(&mut self, factor: &u64) {
        // Inspect the objects that the monkey is holding
        //
        // Modify the worry level of each of the items that the monkey is holding.
        // This method reduces the worry levels by applying the modulo of the minimim common
        // multiple of all the 'divisible_by' values in the whole set of monkeys.
        self.items_worry.iter_mut().for_each(|item_worry| {
            *item_worry = match self.operator.as_str() {
                "+" => *item_worry + self.operation_value,
                "-" => *item_worry - self.operation_value,
                "*" => *item_worry * self.operation_value,
                "**" => *item_worry * *item_worry,
                _ => panic!("Invalid operation '{}'", self.operator),
            };
            *item_worry %= *factor;
            self.inspection_counter += 1;
        });
    }

    pub fn throw(&mut self) -> Vec<(usize, u64)> {
        // Return a vec with monkey number and the worry level of the item being thrown to
        let mut throws = vec![];
        while !self.items_worry.is_empty() {
            let item = self.items_worry.remove(0);
            let receiver = match item % self.divisible_by {
                0 => self.throw_to_if_pass,
                _ => self.throw_to_if_fail,
            };
            throws.push((receiver, item));
        }
        throws
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

fn parse_file(fname: &String) -> Vec<Monkey> {
    // Read data file
    let data = read_file(&fname);
    // Parse lines
    let mut monkeys = vec![];
    for monkey_lines in data.split("Monkey").filter(|l| !l.is_empty()) {
        monkeys.push(parse_single_monkey(monkey_lines));
    }
    monkeys
}

fn parse_single_monkey(monkey_lines: &str) -> Monkey {
    // Transform lines into an iterator
    let mut lines = monkey_lines.lines();
    // First line contains number of the monkey, we don't need it
    lines.next();
    // Read line with the worry levels of the items that the monkey is holding
    let line = lines.next().unwrap();
    let items_worry: Vec<u64> = line
        .split(":")
        .last()
        .unwrap()
        .split(",")
        .map(|s| s.trim().parse().unwrap())
        .collect();
    // Read line with the operation that the monkey performs
    let line = lines.next().unwrap();
    let parts: Vec<&str> = line.split("=").last().unwrap().split_whitespace().collect();
    let (operator, operation_value) = match *parts.last().unwrap() {
        "old" => (String::from("**"), 0),
        _ => (
            String::from(parts[1]),
            parts.last().unwrap().parse().unwrap(),
        ),
    };
    // Read line with the test that the monkey performs
    let line = lines.next().unwrap();
    let divisible_by = line.split_whitespace().last().unwrap().parse().unwrap();
    // Read line with the result if the test passes
    let line = lines.next().unwrap();
    let throw_to_if_pass = line.split_whitespace().last().unwrap().parse().unwrap();
    // Read line with the result if the test fails
    let line = lines.next().unwrap();
    let throw_to_if_fail = line.split_whitespace().last().unwrap().parse().unwrap();
    // Return the monkey
    Monkey {
        items_worry,
        operator,
        operation_value,
        divisible_by,
        throw_to_if_pass,
        throw_to_if_fail,
        inspection_counter: 0,
    }
}

fn solve_part1(fname: &String) -> u64 {
    // Parse input file and get a vec of the monkeys
    let mut monkeys = parse_file(fname);
    // Run 20 rounds of the monkeys' game
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            monkeys[i].inspect();
            let throws = monkeys[i].throw();
            for (receiver, item) in throws {
                monkeys[receiver].items_worry.push(item);
            }
        }
    }
    // // Compute monkey business
    let mut inspections: Vec<u64> = monkeys.iter().map(|m| m.inspection_counter).collect();
    inspections.sort();
    inspections.reverse();
    inspections[0] * inspections[1]
}

fn solve_part2(fname: &String) -> u64 {
    // Parse input file and get a vec of the monkeys
    let mut monkeys = parse_file(fname);
    // Get the minimum common multiple of all the 'divisible_by' primes in the group of monkeys
    let factor = monkeys.iter().map(|m| m.divisible_by).product();
    // Run 10000 rounds of the monkeys' game
    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            monkeys[i].inspect_with_modulo(&factor);
            let throws = monkeys[i].throw();
            for (receiver, item) in throws {
                monkeys[receiver].items_worry.push(item);
            }
        }
    }
    // // Compute monkey business
    let mut inspections: Vec<u64> = monkeys.iter().map(|m| m.inspection_counter).collect();
    inspections.sort();
    inspections.reverse();
    inspections[0] * inspections[1]
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
