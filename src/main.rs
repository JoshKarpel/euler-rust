extern crate euler;

use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let args = euler::Args::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = euler::solve(&args.problem) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
