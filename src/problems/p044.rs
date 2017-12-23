use std::collections::HashSet;

use super::utils;


pub fn solve() -> i64 {
    let upper_bound: u64 = 3000;

    let mut pentagons: HashSet<u64> = HashSet::new();
    for n in 1..upper_bound {
        pentagons.insert(utils::pentagon_number(&n));
    }

    for n in 1..upper_bound {
        let p_n = utils::pentagon_number(&n);
        for m in 1..n {
            let p_m = utils::pentagon_number(&m);

            if pentagons.contains(&(p_n - p_m)) && pentagons.contains(&(p_n + p_m)) {
                return (p_n - p_m) as i64;
            }
        }
    }

    panic!("Couldn't find solution for problem 044. Try increasing upper bound?")
}
