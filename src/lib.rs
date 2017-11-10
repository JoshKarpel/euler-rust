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
    let answer_map: HashMap<&str, i64> = get_answer_map();

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

pub fn get_answer_map() -> HashMap<&'static str, i64> {
    let mut f = File::open("data/answers.txt")
        .expect("Answers file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Couldn't read answers file");

    let mut answer_map: HashMap<&'static str, i64> = HashMap::new();
    for line in contents.split_terminator("\n") {
        let p_and_a: Vec<&str> = line.split_terminator(":").collect();
        let a: i64 = p_and_a[1].parse().unwrap();
        answer_map.insert(p_and_a[0], a);
    }
    //    let pairs: Vec<Vec<&str>> = contents
    //        .split_terminator("\n")
    //        .map(|s| s.split_terminator(":").collect())
    //        .collect();
    //    println!("{:?}", pairs);
    //
    //    let tuples: Vec<(&str, i64)>
    ////        .map(|v: Vec<&'static str>| (v[0], v[1].parse::<i64>().unwrap())).collect();
    ////        .map(|v: Vec<&str>| {
    ////            let ans: i64 = v[1].parse().expect("yep");
    ////            (v[0], ans)
    ////        })
    ////        .collect();
    ////        .fold(HashMap::new(), |mut answers, p_and_a: Vec<&str>| {
    ////            answers.insert(p_and_a[0], p_and_a[1].parse::<i64>().unwrap());
    ////            answers
    ////        });

    println!("{:?}", answer_map);

    answer_map
}
