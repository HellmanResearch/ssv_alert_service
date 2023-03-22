
use std::time::{SystemTime, UNIX_EPOCH};



fn main() {
    // let now = SystemTime::now();
    // let timestamp = now.elapsed();
    let n = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // let n = time::now();
    println!("timestamp: {}", n);
}