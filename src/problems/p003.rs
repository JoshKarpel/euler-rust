use super::utils;

pub fn solve() -> i64 {
    *utils::prime_factorization(600851475143).last().unwrap()
}
