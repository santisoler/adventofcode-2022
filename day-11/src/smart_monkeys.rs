use crate::factorized_number::Factorized;

#[derive(Debug)]
pub struct SmartMonkey {
    pub items: Vec<Factorized>, // stack of items, represented as worry levels.
    pub operation: String,            // operation that should be carried out
    pub test: u32,                    // test if the worry level is divisible by this value
    pub pass: usize,                  // if test pass, throw the item to monkey number 'pass'
    pub fail: usize,                  // if test fail, throw the item to monkey number 'fail'
}

impl SmartMonkey {
    pub fn inspect_objects(&mut self) -> Vec<(usize, Factorized)> {
        // Inspect the objects that the monkey is holding
        //
        // Returns a vec with tuples. Each tuple corresponds to a throw: the first element is the
        // monkey that should receive the item and the second element is the worry level of the
        // item.
        let mut throw_items = vec![];
        while !self.items.is_empty() {
            let item = self.items.remove(0);
            println!("{:?}", item);
            let item = _perform_operation(&item, &self.operation);
            let receiver: usize = match item.is_divisible_by(&self.test) {
                true => self.pass,
                false => self.fail,
            };
            throw_items.push((receiver, item.clone()));
        }
        throw_items
    }
}

fn _perform_operation(item: &Factorized, operation: &String) -> Factorized {
    let operator = operation.split_whitespace().nth(3).unwrap();
    let value = operation.split_whitespace().last().unwrap();
    let value = {
        if value == "old" {
            item.clone()
        } else {
            Factorized::from(&value.parse().unwrap())
        }
    };
    match operator {
        "*" => item.clone() * value,
        "+" => item.clone() + value,
        "-" => item.clone() - value,
        _ => panic!("Invalid operator '{}'", operator),
    }
}

