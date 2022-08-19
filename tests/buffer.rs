use std::sync::{Arc, Mutex};
use std::thread;
use chrono::Local;
use uid_generator_rust::bit_count;
use uid_generator_rust::buffer::RingBuffer;

#[test]
fn buffer() {
    const SIZE :usize= 1024;
    let mut x = RingBuffer::<SIZE>::from(50).unwrap();
    let mut arc = Arc::new(Mutex::new(x));
    let arc1 = arc.clone();
    thread::spawn(move || {
        let start = Local::now().timestamp_nanos();
        for i in 0..SIZE {
            // let x1 = arc1.lock().unwrap().put(i as i64);
            // if !x1 {
            //     loop {
            //         let x2 = arc1.lock().unwrap().put(i as i64);
            //         if x2 {
            //             break;
            //         }
            //     }
            // }
        }
        let end = Local::now().timestamp_nanos();
        println!("put: {}", end - start)
    });

    let mut vec1 = vec![];
    let start = Local::now().timestamp_nanos();
    loop {
        match arc.lock().unwrap().take() {
            None => {
                println!("123123")
            }
            Some(uis) => {
                vec1.push(uis)
            }
        }

        if vec1.len() == SIZE {
            break
        }
    }
    let end = Local::now().timestamp_nanos();
    println!("take: {}", end - start);
    println!("{}", vec1.len())
}