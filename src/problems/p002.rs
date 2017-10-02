pub fn solve() -> i64 {
    let mut fib_nm1 = 1;
    let mut fib_n = 1;
    let mut next;
    let mut acc = 0;

    while fib_n < 4000000 {
        if fib_n % 2 == 0 {
            acc += fib_n;
        }

        next = fib_n + fib_nm1;
        fib_nm1 = fib_n;
        fib_n = next;
    }

    acc
}
