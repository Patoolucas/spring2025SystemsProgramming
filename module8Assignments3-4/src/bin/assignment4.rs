use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
use rand::Rng;

// Define a special value that will signal termination
const TERMINATION_SIGNAL: i32 = -1;

fn main() {
    // Number of items to produce
    const ITEM_COUNT: usize = 20;
    
    // TODO: Create a channel for sending numbers
    let (tx, rx) = mpsc::channel::<i32>();
    let rx = Arc::new(Mutex::new(rx));
    
    // TODO: Create 2 producer threads
    let mut producers = Vec::new();
    let items_per_producer = ITEM_COUNT / 2;
    let remainder = ITEM_COUNT % 2;
    for id in 1..=2 {
        let tx_clone = tx.clone();
        let count = if id == 1 { items_per_producer + remainder } else { items_per_producer };
        let handle = thread::spawn(move || {
            producer(id, tx_clone, count);
        });
        producers.push(handle);
    }
    
    // TODO: Create 3 consumer threads
    let mut consumers = Vec::new();
    for id in 1..=3 {
        let rx_clone = Arc::clone(&rx);
        let handle = thread::spawn(move || {
            consumer(id, rx_clone);
        });
        consumers.push(handle);
    }
    
    // TODO: Wait for all threads to finish
    for handle in producers {
        handle.join().unwrap();
    }
    for _ in 0..3 {
        tx.send(TERMINATION_SIGNAL).unwrap();
    }
    for handle in consumers {
        handle.join().unwrap();
    }
    
    println!("All items have been produced and consumed!");
}

// TODO: Implement producer function
fn producer(id: usize, tx: mpsc::Sender<i32>, item_count: usize) {
    let mut rng = rand::thread_rng();
    for _ in 0..item_count {
        let num = rng.gen_range(0..100);
        println!("Producer {} produced {}", id, num);
        tx.send(num).unwrap();
        thread::sleep(Duration::from_millis(100));
    }
}

// TODO: Implement consumer function
fn consumer(id: usize, rx: Arc<Mutex<mpsc::Receiver<i32>>>) {
    loop {
        let val = {
            let rx_locked = rx.lock().unwrap();
            rx_locked.recv().unwrap()
        };
        if val == TERMINATION_SIGNAL {
            println!("Consumer {} received termination signal", id);
            break;
        } else {
            println!("Consumer {} consumed {}", id, val);
            thread::sleep(Duration::from_millis(150));
        }
    }
}
