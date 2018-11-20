extern crate nix;

use nix::sched::CloneFlags;
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

    Command::new("/proc/self/exe")
        .env_clear()
        .stdout(Stdio::inherit())
        .stdin(Stdio::inherit())
        .stderr(Stdio::inherit())
        .args(&["child", "abc"])
        .status()
        .expect("failed to execute");
}

fn child(run_args: &[String]) {
    println!("Arguments: {:?}", run_args);

    nix::unistd::sethostname("container").expect("hostname set failed");

    nix::unistd::chroot("/ubuntu-rootfs");
    nix::unistd::chdir("/");

    let status = Command::new("bash")
        .stdout(Stdio::inherit())
        .stdin(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .expect("failed to execute");

    assert!(status.success());
}
