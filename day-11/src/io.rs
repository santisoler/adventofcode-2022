use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::monkeys::Monkey;
use crate::smart_monkeys::SmartMonkey;
use crate::factorized_number::Factorized;


pub fn parse_file(fname: &String) -> Vec<Monkey> {
    // Read data file
    let data = _read_file(&fname);
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

pub fn parse_file_with_factorized(fname: &String) -> Vec<SmartMonkey> {
    // Read data file
    let data = _read_file(&fname);
    // Def vec of monkeys
    let mut monkeys = vec![];
    // Def temp variables
    let mut items = vec![];
    let mut operation = String::from("");
    let mut test = 0;
    let mut pass = 0;
    let mut fail = 0;
    // Start parsing the file
    for line in data.lines() {
        if line.is_empty() {
            monkeys.push(SmartMonkey {
                items: build_vec_of_factorized(&items),
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
    monkeys.push(SmartMonkey {
        items: build_vec_of_factorized(&items),
        operation: operation.clone(),
        test: test.clone(),
        pass: pass.clone(),
        fail: fail.clone(),
    });
    monkeys
}

fn build_vec_of_factorized(items: &Vec<u32>) -> Vec<Factorized> {
    let mut factorized_numbers = vec![];
    for item in items {
        factorized_numbers.push(Factorized::from(&item))
    }
    factorized_numbers
}

fn _read_file(fname: &String) -> String {
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
