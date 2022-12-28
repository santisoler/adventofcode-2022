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
    // Def vec of monkeys
    let mut monkeys: Vec<Monkey> = vec![];
    // Def temp variables
    let mut items: Vec<u32> = vec![];
    let mut operation: String = String::from("");
    let mut test: u32 = 0;
    let mut pass: usize = 0;
    let mut fail: usize = 0;
    // Start parsing the file
    for line in data.lines() {
        if line.is_empty() {
            monkeys.push(Monkey {
                items: items.clone(),
                operation: operation.clone(),
                test: test.clone(),
                pass: pass.clone(),
                fail: fail.clone(),
            })
        } else if line.contains("Starting") {
            let numbers = line.split(":").last().unwrap();
            items = numbers
                .split(",")
                .map(|s| s.trim().parse().unwrap())
                .collect();
        } else if line.contains("Operation") {
            operation = line.split(":").last().unwrap().trim().to_string();
        } else if line.contains("Test") {
            test = line
                .split_ascii_whitespace()
                .last()
                .unwrap()
                .parse()
                .unwrap();
        } else if line.contains("If true") {
            pass = line
                .split_ascii_whitespace()
                .last()
                .unwrap()
                .parse()
                .unwrap();
        } else if line.contains("If false") {
            fail = line
                .split_ascii_whitespace()
                .last()
                .unwrap()
                .parse()
                .unwrap();
        }
    }
    // Push the last monkey on EOF
    monkeys.push(Monkey {
        items: items.clone(),
        operation: operation.clone(),
        test: test.clone(),
        pass: pass.clone(),
        fail: fail.clone(),
    });
    monkeys
}

struct Monkey {
    items: Vec<u32>,   // stack of items, each element is its worry level
    operation: String, // operation that should be carried out
    test: u32,         // test if the worry level is divisible by this value
    pass: usize,       // if test pass, throw the item to monkey number 'pass'
    fail: usize,       // if test fail, throw the item to monkey number 'fail'
}

impl Monkey {
    fn inspect_objects(&mut self) -> Vec<(usize, u32)> {
        // Inspect the objects that the monkey is holding
        //
        // Returns a vec with tuples. Each tuple corresponds to a throw: the first element is the
        // monkey that should receive the item and the second element is the worry level of the
        // item.
        let mut throw_items: Vec<(usize, u32)> = vec![];
        for _ in 0..self.items.len() {
            let item = self.items.remove(0);
            let item = perform_operation(&item, &self.operation) / 3;
            let receiver: usize = match item % &self.test {
                0 => self.pass,
                _ => self.fail,
            };
            throw_items.push((receiver, item));
        }
        throw_items
    }
}

fn perform_operation(item: &u32, operation: &String) -> u32 {
    let operator = operation.split_whitespace().nth(3).unwrap();
    let value = operation.split_whitespace().last().unwrap();
    let value: u32 = {
        if value == "old" {
            *item
        } else {
            value.parse().unwrap()
        }
    };
    match operator {
        "*" => item * value,
        "+" => item + value,
        "-" => item - value,
        _ => panic!("Invalid operator '{}'", operator),
    }
}

fn solve_part1(fname: &String) -> u32 {
    // Parse input file and get a vec of the monkeys
    let mut monkeys = parse_file(fname);
    // Define counters of number of inspections per monkey
    let mut inspections: Vec<u32> = vec![0; monkeys.len()];
    // Run 20 rounds of the monkeys' game
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let throws = monkeys[i].inspect_objects();
            for (receiver, item) in throws {
                monkeys[receiver].items.push(item);
                inspections[i] += 1;
            }
        }
    }
    inspections.sort();
    inspections.reverse();
    inspections[0] * inspections[1]
}

fn main() {
    let fname = String::from("data/input");

    // part 1
    let result = solve_part1(&fname);
    println!("Solution to part 1: {}", result);
}
