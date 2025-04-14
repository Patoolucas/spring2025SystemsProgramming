fn track_changes() {
    let mut tracker = 0;
    let mut update = || {
        tracker += 1;
        println!("Counter: {}", tracker);
    };

    update(); //First call
    update(); //Second call
}

fn main() {
    track_changes();
}
