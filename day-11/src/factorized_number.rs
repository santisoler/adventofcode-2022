use std::cmp::min;
use std::{
    collections::HashMap,
    ops::{Add, Mul, Sub},
};

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
    fn test_from() {
        //
        let mut expected = Factorized {
            factors: HashMap::new(),
        };
        expected.factors.insert(2, 6);
        assert_eq!(Factorized::from(&64), expected);
        //
        let mut expected = Factorized {
            factors: HashMap::new(),
        };
        expected.factors.insert(7, 1);
        expected.factors.insert(11, 1);
        assert_eq!(Factorized::from(&77), expected);
        //
        let mut expected = Factorized {
            factors: HashMap::new(),
        };
        expected.factors.insert(2, 3);
        expected.factors.insert(3, 2);
        assert_eq!(Factorized::from(&72), expected);
        //
        let expected = Factorized {
            factors: HashMap::new(),
        };
        assert_eq!(Factorized::from(&1), expected);
    }

    #[test]
    fn test_clean() {
        let mut number = Factorized::from(&64);
        let expected = number.clone();
        number.factors.insert(5, 0);
        number.clean_factors();
        assert_eq!(number, expected);
        //
        let mut number = Factorized::from(&72);
        let expected = number.clone();
        number.clean_factors();
        assert_eq!(number, expected);
    }

    #[test]
    fn test_to_integer() {
        let value = 64;
        let number = Factorized::from(&value);
        assert_eq!(number.to_integer(), value);
        //
        let value = 72;
        let number = Factorized::from(&value);
        assert_eq!(number.to_integer(), value);
        //
        let value = 1;
        let number = Factorized::from(&value);
        assert_eq!(number.to_integer(), value);
    }

    #[test]
    fn test_mul_trait() {
        let a = Factorized::from(&30);
        let b = Factorized::from(&72);
        let c = a * b;
        assert_eq!(c.to_integer(), 30 * 72);
    }

    #[test]
    fn test_add_trait() {
        let a = Factorized::from(&30);
        let b = Factorized::from(&72);
        let c = a + b;
        assert_eq!(c.to_integer(), 30 + 72);
    }

    #[test]
    fn test_sub_trait() {
        let a = Factorized::from(&72);
        let b = Factorized::from(&30);
        let c = a - b;
        assert_eq!(c.to_integer(), 72 - 30);
        let a = Factorized::from(&30);
        let b = Factorized::from(&15);
        let c = a - b;
        assert_eq!(c.to_integer(), 30 - 15);
    }
}

fn build_primes(n: &u32) -> Vec<u32> {
    let mut primes = vec![2];
    for n in 3..n + 1 {
        if primes.iter().all(|p| n % p != 0) {
            primes.push(n);
        }
    }
    primes
}

#[derive(Debug)]
pub struct Factorized {
    factors: HashMap<u32, u32>,
}

impl Factorized {
    pub fn new() -> Factorized {
        Factorized {
            factors: HashMap::new(),
        }
    }

    pub fn from(number: &u32) -> Factorized {
        if *number == 1 {
            return Factorized {
                factors: HashMap::new(),
            };
        }
        let mut factors = HashMap::new();
        let mut number = number.clone();
        while number != 1 {
            for prime in build_primes(&number).iter() {
                if number % prime == 0 {
                    factors.entry(*prime).and_modify(|p| *p += 1).or_insert(1);
                    number /= prime;
                    break;
                }
            }
        }
        Factorized { factors }
    }

    pub fn to_integer(&self) -> u32 {
        let mut result = 1;
        for (factor, power) in self.factors.iter() {
            result *= factor.pow(*power);
        }
        result
    }

    pub fn is_divisible_by(&self, number: &u32) -> bool {
        match self.factors.get(&number) {
            Some(power) => match power {
                0 => false,
                _ => true,
            },
            None => false,
        }
    }

    fn clean_factors(&mut self) {
        self.factors.retain(|_, power| *power != 0);
    }
}

impl PartialEq for Factorized {
    fn eq(&self, other: &Self) -> bool {
        self.factors == other.factors
    }
}

impl Clone for Factorized {
    fn clone(&self) -> Self {
        Self {
            factors: self.factors.clone(),
        }
    }
}

impl Mul for Factorized {
    // The multiplication of factorized numbers is a closed operation.
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut product = self.clone();
        for (factor, power) in other.factors.iter() {
            product.factors.entry(*factor).and_modify(|p| *p += *power).or_insert(*power);
        }
        product
    }
}

impl Add for Factorized {
    // The add of factorized numbers is a closed operation.
    type Output = Self;

    fn add(self, other: Self) -> Self {
        // Find the maximum common divisor between self and other
        let maximum_common_divisor = Self {
            factors: self
                .factors
                .iter()
                .filter(|(k, _)| other.factors.contains_key(k))
                .map(|(k, v)| (k.clone(), min(other.factors.get(k).unwrap(), v).clone()))
                .collect(),
        };
        // Remove the maximum common divisor from self and other (need to clone both because they
        // are not mutable)
        let mut a = self.clone();
        let mut b = other.clone();
        for (factor, power) in maximum_common_divisor.factors.iter() {
            a.factors.entry(*factor).and_modify(|p| *p -= *power);
            b.factors.entry(*factor).and_modify(|p| *p -= *power);
        }
        a.clean_factors();
        b.clean_factors();
        // Sum the residues of a and b as ints and then factorize the result
        let sum_of_residues = Factorized::from(&(a.to_integer() + b.to_integer()));
        maximum_common_divisor * sum_of_residues
    }
}

impl Sub for Factorized {
    // The add of factorized numbers is a closed operation.
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        // Find the maximum common divisor between self and other
        let maximum_common_divisor = Self {
            factors: self
                .factors
                .iter()
                .filter(|(k, _)| other.factors.contains_key(k))
                .map(|(k, v)| (k.clone(), min(other.factors.get(k).unwrap(), v).clone()))
                .collect(),
        };
        // Remove the maximum common divisor from self and other (need to clone both because they
        // are not mutable)
        let mut a = self.clone();
        let mut b = other.clone();
        for (factor, power) in maximum_common_divisor.factors.iter() {
            a.factors.entry(*factor).and_modify(|p| *p -= *power);
            b.factors.entry(*factor).and_modify(|p| *p -= *power);
        }
        a.clean_factors();
        b.clean_factors();
        // Compute difference of the residues of a and b as ints and then factorize the result
        let diff_of_residues = Factorized::from(&(a.to_integer() - b.to_integer()));
        maximum_common_divisor * diff_of_residues
    }
}
