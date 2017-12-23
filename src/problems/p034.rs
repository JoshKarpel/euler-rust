fn get_digits(&n: &u64) -> Vec<u8> {
    n.to_string()
        .split("")
        .filter(|&d| d != "")
        .map(|d| d.parse::<u8>().unwrap())
        .collect::<Vec<u8>>()
}

fn digit_to_factorial(&d: &u8) -> u64 {
    match d {
        0 | 1 => 1,
        2 => 2,
        3 => 6,
        4 => 24,
        5 => 120,
        6 => 720,
        7 => 5_040,
        8 => 40_320,
        9 => 362_880,
        _ => panic!("That's not a digit!")
    }
}

fn get_sum_of_factorials_of_digits(digits: Vec<u8>) -> u64 {
    digits.iter()
        .map(|&d| digit_to_factorial(&d))
        .fold(0, |sum, element| sum + element) as u64
}

fn satisfies_condition(&n: &u64) -> bool {
    n == get_sum_of_factorials_of_digits(get_digits(&n))
}

pub fn solve() -> i64 {
    let upper_bound = digit_to_factorial(&9) * 7;

    (3..upper_bound)
        .filter(|&n| satisfies_condition(&n))
        .fold(0, |sum, element| sum + element) as i64
}
