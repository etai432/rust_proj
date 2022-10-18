pub fn run() {
    let mut count = 0;
    //infinite loop
    loop{
        count += 1;
        print!("{}, ", count);
        if count == 20 { // stop the loop
            break;
        }
    }
    count = 0;
    //while loop (FizzBuzz)
    while count <= 100 {
        if count % 15 == 0 {
            println!("FizzBuzz");
        } else if count % 3 == 0 {
            println!("Fizz");
        } else if count % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", count);
        }
        count += 1;
    }

    //for range
    for x in 0..100 { // 0-99
        print!("{}, ", x);
    }
}