use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicPtr, Ordering};
use std::thread;
use chrono::Local;
use uid_generator_rust::buffer::RingBuffer;

fn main() {


    const SIZE: usize = 131072;
    let ref mut x = RingBuffer::<SIZE>::from(50).unwrap();
    let start = Local::now().timestamp_millis();
    for i in 0..SIZE {
        let x1 = x.put(i as i64);
        if !x1 {
            loop {
                let x2 = x.put(i as i64);
                if x2 {
                    break;
                }
            }
        }
    }

    let end = Local::now().timestamp_millis();
    println!("put: {}", end - start);

    let mut vec1 = vec![];
    let start = Local::now().timestamp_millis();
    loop {
        match x.take() {
            None => {
                // println!("123123")
            }
            Some(uis) => {
                vec1.push(uis)
            }
        }

        if vec1.len() == SIZE {
            break;
        }
    }
    let end = Local::now().timestamp_millis();
    println!("take: {}", end - start);
    println!("{}", vec1.len())
}
