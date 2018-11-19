use std::env;

fn print_usage() {
    println!("usage: progname run <command>");
    println!("    Example: progname run ls");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args[1] == "run" {
        let arguments = &args[1..];
        println!("run main process here, with args: {:?}", &arguments[1..]);
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