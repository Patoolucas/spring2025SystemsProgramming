use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // TODO: Create a shared counter using Arc and Mutex
    let counter = Arc::new(Mutex::new(0));
    
    // TODO: Create a vector to store thread handles
    let mut handles = vec![];
    
    // TODO: Spawn 5 threads
    for _i in 1..=5 {
        // TODO: Clone the Arc for the thread
        let counter_clone = Arc::clone(&counter);
        
        // TODO: Spawn a thread that increments the counter 10 times
        let handle = thread::spawn(move || {
            // TODO: Increment counter 10 times
            for _ in 0..10 {
                let mut num = counter_clone.lock().unwrap();
                *num += 1;
            }
        });
        
        handles.push(handle);
    }
    
    // TODO: Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    // TODO: Print the final value of the counter
    println!("Final counter value: {}", *counter.lock().unwrap());
}
