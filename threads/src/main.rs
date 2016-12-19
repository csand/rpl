use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        "Hello from a thread!"
    });
    println!("{}", handle.join().unwrap());
    let x = 5;
    thread::spawn(move || {
        println!("x is {}", x);
    });
    thread::sleep(Duration::from_millis(50));
}
