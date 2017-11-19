pub fn solve() -> i64 {
    let mut fib_prev = 1;
    let mut fib_curr = 1;
    let mut fib_next;
    let mut acc = 0;

    while fib_curr < 4000000 {
        if fib_curr % 2 == 0 {
            acc += fib_curr;
        }

        fib_next = fib_curr + fib_prev;
        fib_prev = fib_curr;
        fib_curr = fib_next;
    }

    acc
}
