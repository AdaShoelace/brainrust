use std::fs;
use std::io::prelude::*;
use std::env;

pub mod interpreter;
use interpreter::Interpreter;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        Interpreter::new(&args[1], &args[2]).run();
    } else {
        Interpreter::new(&args[1], "0").run();
    }
}
