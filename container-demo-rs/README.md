# Container Demo
This demo is meant to explain the basics of the Rust Programming Language (and maybe Linux containers too.)

The focus is not containers, but Rust.

## Running the Demo
```bash
vagrant up  # This should setup the VM
cargo build # This will build the program
vagrant ssh # ssh into the VM

# now inside the VM
cd /vagrant/target/debug
sudo ./container-demo-rs run ls -l -a # run: "ls -l -a" inside container
```

### Resources
I blatantly ripped off of Liz Rice's talk and book.
[Advanced Container Concepts for Java Applications - Julian Friedman, IBM](https://youtu.be/nTdurlJfDEI)

[How to Containerize Your Go Code](https://www.oreilly.com/library/view/how-to-containerize/9781491982310/)
