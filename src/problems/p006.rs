pub fn solve() -> i64 {
    let n = 101;

    let sum = (1..n)
        .fold(0, |sum, next| sum + next);

    let sum_of_squares = (1..n)
        .fold(0, |sum, next| sum + (next * next));

    (sum * sum) - sum_of_squares
}
