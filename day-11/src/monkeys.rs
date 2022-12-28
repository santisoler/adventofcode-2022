pub struct Monkey {
    pub items: Vec<u32>,   // stack of items, each element is its worry level
    pub operation: String, // operation that should be carried out
    pub test: u32,         // test if the worry level is divisible by this value
    pub pass: usize,       // if test pass, throw the item to monkey number 'pass'
    pub fail: usize,       // if test fail, throw the item to monkey number 'fail'
}

impl Monkey {
    pub fn inspect_objects(&mut self) -> Vec<(usize, u32)> {
        // Inspect the objects that the monkey is holding
        //
        // Returns a vec with tuples. Each tuple corresponds to a throw: the first element is the
        // monkey that should receive the item and the second element is the worry level of the
        // item.
        let mut throw_items: Vec<(usize, u32)> = vec![];
        for _ in 0..self.items.len() {
            let item = self.items.remove(0);
            let item = _perform_operation(&item, &self.operation) / 3;
            let receiver: usize = match item % &self.test {
                0 => self.pass,
                _ => self.fail,
            };
            throw_items.push((receiver, item));
        }
        throw_items
    }
}

fn _perform_operation(item: &u32, operation: &String) -> u32 {
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

