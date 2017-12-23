use std::error::Error;
use std::collections::HashMap;
use std::process;
use std::fs::File;
use std::io::prelude::*;

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

    let answers = read_answers();
    let answer_map: HashMap<String, i64> = parse_answer_map(answers);

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

    let correct = match answer_map.get(problem) {
        Some(c) => {
            match *c == answer {
                true => "✓",
                false => "✘",
            }
        },
        None => "?",
    };

    println!("Answer: {} {} |  Elapsed Time: {:.6} seconds", answer, correct, end - start);

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
    problems.insert("021", problems::p021::solve);
    problems.insert("023", problems::p023::solve);
    problems.insert("044", problems::p044::solve);

    problems
}

pub fn read_answers() -> String {
    let mut f = File::open("data/answers.txt")
        .expect("Answers file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Couldn't read answers file");

    contents
}

pub fn parse_answer_map(answers: String) -> HashMap<String, i64> {
    let mut intermediate: HashMap<&str, &str> = HashMap::new();
    for line in answers.split_terminator("\n") {
        let p_and_a: Vec<&str> = line.split(":").collect();
        intermediate.insert(p_and_a[0], p_and_a[1]);
    }

    let mut answer_map: HashMap<String, i64> = HashMap::new();
    for (problem, answer) in intermediate {
        match answer.parse::<i64>() {
            Ok(x) => { answer_map.insert(problem.to_string(), x); },
            Err(_) => {}
        }
    }

    answer_map
}
