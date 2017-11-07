use std::collections::HashMap;

use super::utils;

pub fn solve() -> i64 {
    let mut target_factorization: HashMap<u64, u32> = HashMap::new();

    for n in 1..21 {
        let n_factorization = utils::hist(utils::prime_factorization(n));
        for (factor, quantity) in n_factorization {
            let q = target_factorization.entry(factor).or_insert(0);
            if quantity > *q {
                *q = quantity;
            }
        }
    }

    utils::num_from_prime_factorization_hist(target_factorization) as i64
}
