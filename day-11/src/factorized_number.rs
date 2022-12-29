use std::cmp;
use std::collections::HashMap;
use std::ops::{Add, Mul, Sub};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_primes() {
        let expected: Vec<u32> = vec![2, 3, 5, 7, 11, 13];
        assert_eq!(build_primes(&13), expected);
        let expected: Vec<u32> = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
        assert_eq!(build_primes(&30), expected);
    }

    #[test]
    fn test_factorize() {
        let value: u32 = 64;
        let mut expected: HashMap<u32, u32> = HashMap::new();
        expected.insert(2, 6);
        assert_eq!(factorize(&value).factors, expected);
        let value: u32 = 77;
        let mut expected: HashMap<u32, u32> = HashMap::new();
        expected.insert(7, 1);
        expected.insert(11, 1);
        assert_eq!(factorize(&value).factors, expected);
        let value: u32 = 72;
        let mut expected: HashMap<u32, u32> = HashMap::new();
        expected.insert(2, 3);
        expected.insert(3, 2);
        assert_eq!(factorize(&value).factors, expected);
    }

    #[test]
    fn test_clean() {
        let mut factors: HashMap<u32, u32> = HashMap::new();
        factors.insert(2, 3);
        factors.insert(3, 2);
        factors.insert(5, 0);
        let mut number = FactorizedNumber { factors };
        number.clean();
        let mut expected: HashMap<u32, u32> = HashMap::new();
        expected.insert(2, 3);
        expected.insert(3, 2);
        assert_eq!(number.factors, expected);
    }

    #[test]
    fn test_to_int() {
        let original: u32 = 30;
        let n = factorize(&original);
        assert_eq!(n.to_int(), original);
    }

    #[test]
    fn test_mul_trait() {
        let a = factorize(&30);
        let b = factorize(&72);
        let c = &a * &b;
        assert_eq!(c.to_int(), 30 * 72);
    }

    #[test]
    fn test_add_trait() {
        let a = factorize(&30);
        let b = factorize(&72);
        let c = &a + &b;
        assert_eq!(c.to_int(), 30 + 72);
    }

    #[test]
    fn test_sub_trait() {
        let a = factorize(&72);
        let b = factorize(&30);
        let c = &a - &b;
        assert_eq!(c.to_int(), 72 - 30);
    }
}

#[derive(Debug)]
pub struct FactorizedNumber {
    // Represent an int by its factors through a hashmap.
    //
    // The keys of the hashmap are the primes factors of the integer and the values are the
    // corresponding power for each factor.
    factors: HashMap<u32, u32>,
}

impl FactorizedNumber {
    pub fn to_int(&self) -> u32 {
        // Converts factorized number to int
        defactorize(&self.factors)
    }

    pub fn clean(&mut self) {
        // Removes any factors that have a zero power
        clean(&mut self.factors);
    }
}

impl Mul<&FactorizedNumber> for &FactorizedNumber {
    // Implement the Mul trait for FactorizedNumber
    type Output = FactorizedNumber;

    fn mul(self, other: &FactorizedNumber) -> FactorizedNumber {
        let mut factors = self.factors.clone();
        for (prime, power) in other.factors.iter() {
            factors
                .entry(*prime)
                .and_modify(|p| *p += *power)
                .or_insert(1);
        }
        FactorizedNumber { factors }
    }
}

impl Add<&FactorizedNumber> for &FactorizedNumber {
    // Implement the Add trait for FactorizedNumber
    type Output = FactorizedNumber;

    fn add(self, other: &FactorizedNumber) -> FactorizedNumber {
        let mut a = self.factors.clone();
        let mut b = other.factors.clone();
        let mut common_factor: HashMap<u32, u32> = HashMap::new();
        let common_primes: Vec<&u32> = self
            .factors
            .keys()
            .filter(|f| other.factors.contains_key(f))
            .collect();
        for prime in common_primes.iter() {
            let minimum: i32 = cmp::min(
                a.get(prime).unwrap().to_owned() as i32,
                b.get(prime).unwrap().to_owned() as i32,
            );
            common_factor.insert(**prime, minimum as u32);
            a.entry(**prime).and_modify(|p| *p -= minimum as u32);
            b.entry(**prime).and_modify(|p| *p -= minimum as u32);
        }
        // Remove any null power from a and b
        clean(&mut a);
        clean(&mut b);
        // Create FactorizedNumber for the common_factor and for the rest
        let rest = factorize(&(defactorize(&a) + defactorize(&b)));
        let common_factor = FactorizedNumber {
            factors: common_factor,
        };
        // Return the multiplication of both
        &rest * &common_factor
    }
}

impl Sub<&FactorizedNumber> for &FactorizedNumber {
    // Implement the Sub trait for FactorizedNumber
    type Output = FactorizedNumber;

    fn sub(self, other: &FactorizedNumber) -> FactorizedNumber {
        let mut a = self.factors.clone();
        let mut b = other.factors.clone();
        let mut common_factor: HashMap<u32, u32> = HashMap::new();
        let common_primes: Vec<&u32> = self
            .factors
            .keys()
            .filter(|f| other.factors.contains_key(f))
            .collect();
        for prime in common_primes.iter() {
            let minimum: i32 = cmp::min(
                a.get(prime).unwrap().to_owned() as i32,
                b.get(prime).unwrap().to_owned() as i32,
            );
            common_factor.insert(**prime, minimum as u32);
            a.entry(**prime).and_modify(|p| *p -= minimum as u32);
            b.entry(**prime).and_modify(|p| *p -= minimum as u32);
        }
        // Remove any null power from a and b
        clean(&mut a);
        clean(&mut b);
        // Create FactorizedNumber for the common_factor and for the rest
        let rest = factorize(&(defactorize(&a) - defactorize(&b)));
        let common_factor = FactorizedNumber {
            factors: common_factor,
        };
        // Return the multiplication of both
        &rest * &common_factor
    }
}

fn build_primes(max: &u32) -> Vec<u32> {
    // Build a list of all primes lower or equal than 'max'
    let mut primes: Vec<u32> = vec![2];
    for n in 3..max + 1 {
        if primes.iter().all(|p| n % p != 0) {
            primes.push(n);
        }
    }
    primes
}

fn factorize(number: &u32) -> FactorizedNumber {
    // Factorize a number into its prime factors
    let primes = build_primes(&number);
    let mut tmp = *number;
    let mut factors: HashMap<u32, u32> = HashMap::new();
    while tmp != 1 {
        for prime in primes.iter() {
            if tmp % prime == 0 {
                tmp /= prime;
                factors
                    .entry(*prime)
                    .and_modify(|power| *power += 1)
                    .or_insert(1);
                break;
            }
        }
    }
    FactorizedNumber { factors }
}

fn defactorize(number: &HashMap<u32, u32>) -> u32 {
    let mut result: u32 = 1;
    for (prime, power) in number.iter() {
        result *= prime.pow(*power);
    }
    result
}

fn clean(number: &mut HashMap<u32, u32>) {
    // Removes any factors that have a zero power
    let null_powers: Vec<u32> = number
        .keys()
        .filter(|p| *number.get(p).unwrap() == 0)
        .cloned()
        .collect();
    for key in null_powers.iter() {
        number.remove(key);
    }
}
