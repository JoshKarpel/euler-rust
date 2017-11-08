use std::error::Error;
use std::collections::HashMap;
use std::process;

extern crate time;

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

pub fn solve(problem: &str) -> Result<(), Box<Error>> {
    let mut problems: HashMap<&str, fn() -> i64> = HashMap::new();
    problems.insert("001", problems::p001::solve);
    problems.insert("002", problems::p002::solve);
    problems.insert("003", problems::p003::solve);
    problems.insert("005", problems::p005::solve);
    problems.insert("006", problems::p006::solve);
    problems.insert("009", problems::p009::solve);

    let solver = match problems.get(problem) {
        Some(solver) => solver,
        None => {
            println!("Couldn't find solver for problem {}!", problem);
            process::exit(0)
        }
    };

    let start = time::precise_time_s();
    let answer = solver();
    let end = time::precise_time_s();

    println!("Answer: {}  |  Elapsed Time: {:.9} seconds", answer, end - start);

    Ok(())
}

