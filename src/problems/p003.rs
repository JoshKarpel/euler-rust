use super::utils;

pub fn solve() -> i64 {
    *utils::prime_factorization(600_851_475_143).last().unwrap() as i64
}
