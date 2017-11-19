use super::utils;


fn sum_of_proper_factors(&n: &u64) -> u64 {
    utils::proper_factorization(n).iter().sum()
}


fn find_amicable_pair(a: u64) -> Option<(u64, u64)> {
    let b: u64 = sum_of_proper_factors(&a);

    if a == b {
        return None;
    }

    if a == sum_of_proper_factors(&b) {
        return Some((a, b));
    }

    None
}

pub fn solve() -> i64 {
    let mut amicable_numbers: Vec<u64> = Vec::new();

    for a in 1..10000 {
        match find_amicable_pair(a) {
            Some((a, b)) => {
                amicable_numbers.push(a);
                amicable_numbers.push(b);
            }
            None => {}
        }
    }

    amicable_numbers.sort();
    amicable_numbers.dedup();

    let answer: u64 = amicable_numbers
        .iter()
        .sum();
    answer as i64
}
