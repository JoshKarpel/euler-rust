pub fn prime_factorization(&n: &i64) -> Vec<i64> {
    let mut test: i64 = n;
    let mut factors: Vec<i64> = Vec::new();

    while test % 2 == 0 {
        factors.push(2);
        test /= 2;
    }

    let mut x: i64 = 3;

    while test > 1 {
        if test % x == 0 {
            factors.push(x);
            test /= x;
        } else {
            x += 2;
        }
    }

    return factors;
}
