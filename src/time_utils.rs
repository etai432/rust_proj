// use chrono;
#[macro_export]
// #[macro_use]
macro_rules! timeit {
    ($($todo: stmt), *) => {
        use std::time::SystemTime;
        let start = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
        $(
            $todo
        )*
        let end = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
        let sec = end.as_secs() - start.as_secs();
        if sec > 1 {
            if sec > 60 {
                println!("timeit result: {} minutes and {} seconds", sec % 60, sec / 60);
            }
            else {
                println!("timeit results: {} seconds", sec);
            }
        }
        else {
            let millis = end.as_millis() - start.as_millis();
            if millis > 1 {
                println!("timeit results: {} milliseconds", millis);
            }
            else {
                let micros = end.as_micros() - start.as_micros();
                if micros > 1 {
                    println!("timeit results: {} microseconds", micros);
                }
                else{
                    println!("timeit results: {} nanoseconds", end.as_nanos() - start.as_nanos())
                }
            }
        }
    };
}

pub fn test() {
    timeit!(something());
}

pub fn something() {

}

//functions to make: timeit (might be a macro)
//