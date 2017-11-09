use std::error::Error;
use std::collections::HashMap;
use std::process;

extern crate time;

pub mod problems;

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
    let solver_map: HashMap<&str, fn() -> i64> = get_solver_map();

    let solver = match solver_map.get(problem) {
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

pub fn get_solver_map() -> HashMap<&'static str, fn() -> i64> {
    let mut problems: HashMap<&str, fn() -> i64> = HashMap::new();
    problems.insert("001", problems::p001::solve);
    problems.insert("002", problems::p002::solve);
    problems.insert("003", problems::p003::solve);
    problems.insert("005", problems::p005::solve);
    problems.insert("006", problems::p006::solve);
    problems.insert("009", problems::p009::solve);
    problems.insert("010", problems::p010::solve);
    problems.insert("012", problems::p012::solve);
    problems.insert("014", problems::p014::solve);

    problems
}
