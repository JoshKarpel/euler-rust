use std::collections::HashMap;
use std::hash::Hash;
use std::cmp::Eq;

pub fn prime_factorization(mut n: i64) -> Vec<i64> {
    let mut factors: Vec<i64> = Vec::new();

    while n % 2 == 0 {
        factors.push(2);
        n /= 2;
    }

    let mut x: i64 = 3;

    while n > 1 {
        if n % x == 0 {
            factors.push(x);
            n /= x;
        } else {
            x += 2;
        }
    }

    return factors;
}

pub fn hist<T: Hash + Eq>(v: Vec<T>) -> HashMap<T, u64> {
    let mut h: HashMap<T, u64> = HashMap::new();

    for val in v {
        *h.entry(val).or_insert(0) += 1;
    }

    return h;
}
