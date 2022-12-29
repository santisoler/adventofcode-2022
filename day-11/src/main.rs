mod io;
mod monkeys;
mod smart_monkeys;
mod factorized_number;

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

fn solve_part1(fname: &String) -> u32 {
    // Parse input file and get a vec of the monkeys
    let mut monkeys = io::parse_file(fname);
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
    // Compute monkey business
    inspections.sort();
    inspections.reverse();
    inspections[0] * inspections[1]
}

fn solve_part2(fname: &String) -> u32 {
    // Parse input file and get a vec of the monkeys
    let mut monkeys = io::parse_file_with_factorized(fname);
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
    println!("{:?}", inspections);
    // Compute monkey business
    inspections.sort();
    inspections.reverse();
    inspections[0] * inspections[1]
}

fn main() {
    // let fname = String::from("data/input");
    let fname = String::from("data/test_input");

    // part 1
    let result = solve_part1(&fname);
    println!("Solution to part 1: {}", result);

    // part 2
    let result = solve_part2(&fname);
    println!("Solution to part 2: {}", result);
}
