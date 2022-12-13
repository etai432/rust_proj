use std::cmp::Reverse;
use std::thread;
// use std::time::Duration;
use std::sync::mpsc;

pub fn run() {
    // let v = vec![1, 2, 3];

    // let handle = thread::spawn(move || {
    //     for i in 1..10 {
    //         println!("spawned: {}", i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    //     println!("vector: {:?}", v)
    // });

    // //cannot use v after it was moved

    // for i in 1..5 {
    //     println!("main: {}", i);
    //     thread::sleep(Duration::from_millis(1));
    // }

    // handle.join().unwrap(); // waits for the handle thread to finish

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let msg = String::from("hi");
        tx.send(msg).unwrap();
    }); 

    let received = rx.recv().unwrap();
    println!("msg: {}", received)
}