use std::{thread, time};

fn main() {
    let millis = time::Duration::from_millis(500);
    let mut v = Vec::with_capacity(4);
    let mut capacity: usize = 4;

    for i in 0..15 {
        v.push(i);
        if v.capacity() == capacity {
            println!("                   ============");
            capacity *= 2;
        }
        println!(
            "ptr:{:?},capacity:{:?},len:{:?},value:{:?}",
            v.as_ptr(),
            format!("{: >2}", v.capacity().to_string()),
            format!("{: >2}", v.len().to_string()),
            v
        );
        thread::sleep(millis);
    }
}
