use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let a = Arc::new(Mutex::new(vec![1u32, 2]));
    let a2 = a.clone(); // ref count bump
    let child = thread::spawn(move || {
        let mut vector = a2.lock().unwrap();
        vector.push(4);
        vector.push(5);
    });

    let _ = child.join();
    println!("{}", a.lock().unwrap().len());
}
