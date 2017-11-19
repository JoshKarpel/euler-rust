use std::collections::HashMap;
use std::hash::Hash;
use std::cmp::Eq;

extern crate bit_vec;

pub fn full_factorization(n: u64) -> Vec<u64> {
    let mut factors: Vec<u64> = Vec::new();

    for x in 1..(((n + 1) as f64).sqrt() as u64 + 1) {
        if n % x == 0 {
            factors.push(x);
            factors.push(n / x);
        }
    }

    factors.sort();
    factors.dedup();
    factors
}

pub fn proper_factorization(n: u64) -> Vec<u64> {
    let mut factors = full_factorization(n);
    factors.pop();
    factors
}

pub fn prime_factorization(mut n: u64) -> Vec<u64> {
    let mut factors: Vec<u64> = Vec::new();

    while n % 2 == 0 {
        factors.push(2);
        n /= 2;
    }

    let mut divisor: u64 = 3;

    while n > 1 {
        if n % divisor == 0 {
            factors.push(divisor);
            n /= divisor;
        } else {
            divisor += 2;
        }
    }

    return factors;
}


pub fn count<T: Hash + Eq>(v: Vec<T>) -> HashMap<T, u32> {
    let mut h: HashMap<T, u32> = HashMap::new();

    for val in v {
        *h.entry(val).or_insert(0) += 1;
    }

    return h;
}


pub fn num_from_prime_factorization_hist(h: HashMap<u64, u32>) -> u64 {
    h.iter()
        .fold(1, |x, (factor, quantity)| x * factor.pow(*quantity))
}


pub fn sieve_of_eratosthenes(n: usize) -> Vec<u64> {
    let mut sieve = bit_vec::BitVec::from_elem(n + 1, true);
    sieve.set(0, false);
    sieve.set(1, false);

    let mut x: usize;

    for p in 2..n {
        if sieve[p] {
            x = p * p;
            while x <= n {
                sieve.set(x, false);
                x += p;
            }
        }
    }

    (0..n as u64).filter(|&x| sieve[x as usize]).collect()
}

pub fn triangle_number(&n: &u64) -> u64 {
    n * (n + 1) / 2
}

pub fn pentagon_number(&n: &u64) -> u64 {
    n * ((3 * n) - 1) / 2
}

pub fn hexagon_number(&n: &u64) -> u64 {
    n * ((2 * n) - 1)
}
