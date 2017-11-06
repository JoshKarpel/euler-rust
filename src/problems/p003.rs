use super::utils;

pub fn solve() -> i64 {
    let factors = utils::prime_factorization(&600851475143);
    *factors.last().unwrap()
}
