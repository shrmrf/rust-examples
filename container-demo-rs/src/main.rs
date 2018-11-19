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
        child(&arguments[..]);
    } else {
        print_usage();
    }
}

fn run(run_args: &[String]) {
    println!("Arguments: {:?}", run_args);

    nix::sched::unshare(
        CloneFlags::CLONE_NEWUTS | CloneFlags::CLONE_NEWPID | CloneFlags::CLONE_NEWNS,
    );

    // Requires `root`
    nix::unistd::sethostname("container").expect("hostname set failed");
    Command::new("/proc/self/exe")
        .stdout(Stdio::inherit())
        .args(&["child", "abc"])
        .output()
        .expect("failed to execute");
}

fn child(run_args: &[String]) {
    println!("Arguments: {:?}", run_args);

    nix::unistd::chroot("/home/taimoor/dev/rust-examples/container-demo-rs/rootfs");
    nix::unistd::chdir("/");
    Command::new("/bin/ls")
        .stdout(Stdio::inherit())
        .args(&["-l", "-a"])
        .output()
        .expect("failed to execute");
}
