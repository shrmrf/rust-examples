extern crate nix;

use nix::sched::{self, CloneFlags};
use std::env;

fn print_usage() {
    println!("usage: progname run <command>");
    println!("    Example: progname run ls");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let arguments = &args[1..];
    if args[1] == "run" {
        run(&arguments[1..]);
    } else if args[1] == "child" {
        println!("run child process here");
    } else {
        print_usage();
    }
}

fn run(run_args: &[String]) {
    println!("Arguments: {:?}", run_args);
}
