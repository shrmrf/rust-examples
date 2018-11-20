extern crate nix;

use nix::sched::CloneFlags;
use std::env;
use std::process::Command;

fn print_usage() {
    println!("usage: progname run <command>");
    println!("    Example: progname run ls");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args[1].as_ref() {
        "run" => run(&args[2..]),
        "child" => child(&args[2..]),
        _ => print_usage(),
    }
}

fn run(run_args: &[String]) {
    nix::sched::unshare(
        CloneFlags::CLONE_NEWUTS | CloneFlags::CLONE_NEWPID | CloneFlags::CLONE_NEWNS,
    ).expect("unshare failed");

    let mut child_args: Vec<String> = vec!["child".to_string()];
    child_args.extend_from_slice(run_args);

    Command::new("/proc/self/exe")
        .env_clear()
        .args(&child_args)
        .status()
        .expect("failed to execute child process");
}

fn child(run_args: &[String]) {
    println!("Arguments: {:?}", run_args);

    nix::unistd::sethostname("container").expect("hostname set failed");
    nix::unistd::chroot("/ubuntu-rootfs").expect("chroot failed");
    nix::unistd::chdir("/").expect("chdir failed");

    Command::new(&run_args[0])
        .args(&run_args[1..])
        .status()
        .expect("failed to execute container");
}
