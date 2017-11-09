use super::utils;

pub fn solve() -> i64 {
    let mut n = 0;
    loop {
        n += 1;

        if utils::full_factorization(utils::triangle_number(&n)).len() > 500 {
            break
        }
    }
    utils::triangle_number(&n) as i64
}
