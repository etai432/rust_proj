use std::time::SystemTime;
// use chrono;

pub fn test() {
    println!("{:?}", SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos());
}

macro_rules! timeit {
    () => {
        
    };
}

pub fn 

//functions to make: timeit (might be a macro)
//