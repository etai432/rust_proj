use std::thread;
use std::time::Duration;
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
    let tx2 = tx.clone();
    thread::spawn(move || {
        let vals = vec!["hi", "hello", "test"];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec!["hi1", "hello1", "test1"];
        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    }); 

    for received in rx {
        println!("msg: {}", received)
    }
}