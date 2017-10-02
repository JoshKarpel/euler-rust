use std::error::Error;
use std::collections::HashMap;
use std::time::Instant;
use std::process;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}

mod problems;

pub struct Args {
    pub problem: String,
}

impl Args {
    pub fn new(args: &[String]) -> Result<Args, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments!");
        }
        let problem = args[1].clone();

        Ok(Args { problem })
    }
}

pub fn run(problem: &str) -> Result<(), Box<Error>> {
    let mut problems: HashMap<&str, fn() -> i64> = HashMap::new();
    problems.insert("001", problems::p001::solve);
    problems.insert("002", problems::p002::solve);

    let solver = match problems.get(problem) {
        Some(solver) => solver,
        None => {
            println!("Couldn't find solver for problem {}!", problem);
            process::exit(0)
        }
    };

    let start = Instant::now();
    let answer = solver();
    let end = Instant::now();

    let duration = end.duration_since(start);
    let duration_in_sec = duration.as_secs() as f64 + duration.subsec_nanos() as f64 * 1e-9;

    println!("Answer: {}  |  Elapsed Time: {:.6} seconds", answer, duration_in_sec);

    Ok(())
}

