use std::collections::HashSet;
use std::iter::FromIterator;

use super::utils;

fn sum_of_proper_divisors(n: &u64) -> u64 {
    let proper_factors = utils::proper_factorization(*n);

    proper_factors.iter()
        .fold(0, |sum, &element| sum + element)
}

fn is_abundant(n: &u64) -> bool {
    sum_of_proper_divisors(&n) > *n
}

pub fn solve() -> i64 {
    let upper_bound: u64 = 28123;
//    let abundant_numbers: HashSet<u64> = HashSet::from_iter((1..upper_bound + 1).filter(|&n| is_abundant(&n)));
//    let mut abundant_sums: HashSet<u64> = HashSet::new();
    let abundant_numbers: Vec<u64> = (1..upper_bound + 1).filter(|&n| is_abundant(&n)).collect();
    let mut abundant_sums: Vec<u64> = Vec::new();

    for n in &abundant_numbers {
        for m in &abundant_numbers {
            if m > n { continue }
//            println!("{}, {}", n, m);
//            abundant_sums.insert(n + m);
            abundant_sums.push(n + m);
        }
    }

    abundant_sums.sort();
    abundant_sums.dedup();
    let abundant_sums: HashSet<u64> = HashSet::from_iter(abundant_sums);
    HashSet::from_iter(1..upper_bound + 1)
        .difference(&abundant_sums)
        .fold(0, |sum, &element| sum + element) as i64
}
