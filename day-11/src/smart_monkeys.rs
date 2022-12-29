use crate::factorized_number::Factorized;

pub struct SmartMonkey {
    pub items: Vec<Factorized>, // stack of items, represented as worry levels.
    pub operation: String,            // operation that should be carried out
    pub test: u32,                    // test if the worry level is divisible by this value
    pub pass: usize,                  // if test pass, throw the item to monkey number 'pass'
    pub fail: usize,                  // if test fail, throw the item to monkey number 'fail'
}
