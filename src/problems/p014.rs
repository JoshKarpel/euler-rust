use std::collections::HashMap;

fn collatz(n: u64) -> u64 {
    if n % 2 == 0 {
        n / 2
    } else {
        (3 * n) + 1
    }
}

pub fn solve() -> i64 {
    let mut collatz_map: HashMap<u64, u64> = HashMap::new();

    let mut n;
    let mut len;

    for start in 1..1000000 {
        n = start;
        len = 1;

        while n != 1 {
            n = collatz(n);
            match collatz_map.get(&n) {
                Some(x) => {
                    len += *x;
                    break;
                }
                None => len += 1
            }
        }

        collatz_map.insert(start, len);
    }

    *collatz_map.iter()
        .max_by_key(|&(&_, &value)| value)
        .unwrap()
        .0 as i64  // get first element of tuple
}
