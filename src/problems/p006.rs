pub fn solve() -> i64 {
    let n = 101;

    let mut square_of_sum = (1..n)
        .fold(0, |a, b| a + b);
    square_of_sum *= square_of_sum;

    let sum_of_squares = (1..n)
        .fold(0, |a, b| a + (b * b));

    square_of_sum - sum_of_squares
}
