pub fn solve() -> i64 {
    let limit = 1000;

    for a in 1..limit {
        for b in 1..(limit - a) {
            let c = limit - a - b;

            if (a * a) + (b * b) == (c * c) {
                return a * b * c as i64;
            }
        }
    }

    panic!("Didn't find matching triplet")
}
