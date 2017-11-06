use std::collections::HashMap;

use super::utils;

pub fn solve() -> i64 {
    let mut target_factorization: HashMap<i64, u64> = HashMap::new();

    for x in 1..21 {
        let x_factorization = utils::hist(utils::prime_factorization(x));
        for (factor, quantity) in x_factorization {
            let q = target_factorization.entry(factor).or_insert(0);
            if quantity > *q {
                *q = quantity;
            }
        }
    }

    let mut target = 1;

    for (factor, quantity) in target_factorization {
        target *= (factor as u32).pow(quantity as u32);
    }

    return target as i64;
}
