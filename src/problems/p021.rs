use super::utils;


fn sum_of_proper_factors(&n: &u64) -> u64 {
    utils::proper_factorization(n).iter().sum()
}

fn is_pair_amicable(&a: &u64, &b: &u64) -> bool {
    a == sum_of_proper_factors(&b) && b == sum_of_proper_factors(&a)
}

pub fn solve() -> i64 {
    println!("{}", is_pair_amicable(&220, &284));

    let mut amicable_numbers: Vec<u64> = Vec::new();

    for a in 1..10000 {
        for b in a + 1..10000 {
            if is_pair_amicable(&a, &b) {
                println!("{}, {}", a, b);
                amicable_numbers.push(a as u64);
            }
        }
    }

    let answer: u64 = amicable_numbers.iter().sum();
    answer as i64
}
