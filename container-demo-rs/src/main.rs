extern crate nix;

use nix::sched::{self, CloneFlags};
use std::env;
use std::process::{Command, Stdio};

fn print_usage() {
    println!("usage: progname run <command>");
    println!("    Example: progname run ls");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);
    let arguments = &args[1..];
    if args[1] == "run" {
        run(&arguments[..]);
    } else if args[1] == "child" {
        println!("run child process here");
    } else {
        print_usage();
    }
}

fn run(run_args: &[String]) {
    println!("Arguments: {:?}", run_args);

    Command::new("/proc/self/exe")
        .stdout(Stdio::inherit())
        .args(&["child", "abc"])
        .output()
        .expect("failed to execute");
}
