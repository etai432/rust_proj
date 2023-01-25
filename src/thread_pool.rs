use std::{thread, task};

pub fn run(tasks:Vec<Box<dyn Fn()>>) {
    for task in tasks {
        thread::spawn(move || {*task});
    }
}
