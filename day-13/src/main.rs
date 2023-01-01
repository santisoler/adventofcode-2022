use std::io::Read;
use std::{cmp::Ordering, fs::File, path::Path};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equal_values() {
        let a = Packet::from("[1,2,3]");
        let b = Packet::from("[1,2,3]");
        assert!(a == b);
        assert!(b == a);
        let a = Packet::from("[1,[2,3,4],5]");
        let b = Packet::from("[1,[2,3,4],5]");
        assert!(a == b);
        assert!(b == a);
        let a = Packet::from("[]");
        let b = Packet::from("[]");
        assert!(a == b);
        assert!(b == a);
        let a = Packet::from("[1,[2,[3,4],5,6],7]");
        let b = Packet::from("[1,[2,[3,4],5,6],7]");
        assert!(a == b);
        assert!(b == a);
        let a = Packet::from("[9]");
        let b = Packet::from("[[8]]");
        assert!(a != b);
        assert!(b != a);
        let a = Packet::from("[1,3,3]");
        let b = Packet::from("[1,2,3]");
        assert!(a != b);
        assert!(b != a);
        let a = Packet::from("[1,[3,3]]");
        let b = Packet::from("[1,[2,3]]");
        assert!(a != b);
        assert!(b != a);
    }

    #[test]
    fn test_ord_values() {
        let a = Packet::from("1");
        let b = Packet::from("2");
        assert!(a < b);
        assert!(b > a);
        let a = Packet::from("[1]");
        let b = Packet::from("[2]");
        assert!(a < b);
        let a = Packet::from("[1]");
        let b = Packet::from("[1,1]");
        assert!(a < b);
        let a = Packet::from("[2]");
        let b = Packet::from("[1]");
        assert!(!(a < b));
        let a = Packet::from("[1,0]");
        let b = Packet::from("[2]");
        assert!(a < b);
        let a = Packet::from("[4,0]");
        let b = Packet::from("[2]");
        assert!(!(a < b));
        let a = Packet::from("[[1],[2,3,4]]");
        let b = Packet::from("[[1],4]");
        assert!(a < b);
    }

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
        assert_eq!(result, 140);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Ord)]
pub enum Packet {
    Integer(u64),
    List(Vec<Self>),
}

impl Packet {
    pub fn from(string: &str) -> Packet {
        // Build a Value based on its string representation.
        //
        // This string could either be something between brackets, like "[1,2,3]", "[]" or
        // "[[1,2],[3,4],5]"; or a single integer, like "93".

        // Parse an int
        if string.chars().nth(0).unwrap() != '[' {
            return Packet::Integer(string.parse().unwrap());
        }
        // Parse elements
        let string = &string[1..string.len() - 1]; // remove the brackets and keep the inside
        let mut elements = vec![];
        let mut bracket_state = 0;
        let mut start: usize = 0;
        for (i, char) in string.chars().enumerate() {
            match char {
                '[' => {
                    bracket_state += 1;
                }
                ']' => {
                    bracket_state -= 1;
                }
                ',' => {
                    if bracket_state == 0 {
                        elements.push(Packet::from(&string[start..i]));
                        start = i + 1;
                    }
                }
                ' ' => panic!("Packet strings shouldn't contain spaces!"),
                _ => continue,
            }
        }
        // Add the last element to the list (if any)
        if !&string[start..].is_empty() {
            elements.push(Packet::from(&string[start..]));
        }
        Packet::List(elements)
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Define partial_cmp for Packets.
        //
        // When both packets are lists we can reuse the partial_cmp from the Vec struct: that trait
        // will call this partial_cmp again when it needs to compare the contained Packets.
        match (self, other) {
            (Self::Integer(n), Self::Integer(m)) => Some(n.cmp(m)),
            (Self::Integer(_), Self::List(_)) => Self::List(vec![self.clone()]).partial_cmp(other),
            (Self::List(_), Self::Integer(_)) => self.partial_cmp(&Self::List(vec![other.clone()])),
            (Self::List(left), Self::List(right)) => left.partial_cmp(&right),
        }
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

pub fn solve_part1(fname: &String) -> u64 {
    // Read file
    let content = read_file(&fname);
    // Compare pairs of packets
    let mut sum_of_indices = 0;
    for (i, packets) in content.split("\n\n").filter(|x| !x.is_empty()).enumerate() {
        let mut lines = packets.lines();
        let left = Packet::from(lines.next().unwrap());
        let right = Packet::from(lines.next().unwrap());
        if left < right {
            sum_of_indices += i + 1; // sum i + 1 because i starts counting in zero
        }
    }
    sum_of_indices as u64
}

pub fn solve_part2(fname: &String) -> u64 {
    // Read file
    let content = read_file(&fname);
    // Create a list of all the packets in the input file
    let mut packets: Vec<Packet> = content
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| Packet::from(&l))
        .collect();
    // Insert the two divider packets into the list
    let divider_two = Packet::from("[[2]]");
    let divider_six = Packet::from("[[6]]");
    packets.push(divider_two.clone());
    packets.push(divider_six.clone());
    // Sort the packets
    packets.sort();
    // Find indices of the two divider packets
    let position_two = packets.iter().position(|p| *p == divider_two).unwrap() + 1;
    let position_six = packets.iter().position(|p| *p == divider_six).unwrap() + 1;
    (position_two * position_six) as u64
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
