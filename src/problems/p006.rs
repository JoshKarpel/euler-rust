pub fn solve() -> i64 {
    let n = 101;

    let sum = (1..n)
        .fold(0, |a, b| a + b);

    let sum_of_squares = (1..n)
        .fold(0, |a, b| a + (b * b));

    (sum * sum) - sum_of_squares
}
