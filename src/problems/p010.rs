use super::utils;

pub fn solve() -> i64 {
    let primes = utils::sieve_of_eratosthenes(2000000);

    primes.iter()
        .fold(0, |sum, &element| sum + element as i64)
}
