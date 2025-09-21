// This example demonstrates basic multithreading in Rust using the standard library.
// It spawns a new thread that prints messages while the main thread also prints its own messages
// concurrently. The `join` method is used to ensure the main thread waits for the spawned thread
// to finish before exiting.

use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    let v = vec![1, 2, 3];
    thread::spawn(move || {
        // The `move` keyword is used to transfer ownership of `v` into the closure.
        // This is necessary because the spawned thread may outlive the main thread,
        // and we want to ensure that `v` is valid for the duration of the thread's execution.
        // If we didn't use `move`, the closure would borrow `v`, which could lead to a dangling reference
        // if the main thread finishes and `v` goes out of scope before the spawned thread uses it.
        println!("Here's a vector: {v:?}");
    });

    // If we uncomment the call join here, the main thread will wait for the spawned thread to finish
    // before continuing. This means that the main thread will print all its messages only after
    // the spawned thread has completed its execution.

    // handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
