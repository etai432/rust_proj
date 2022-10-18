pub fn run() {
    //fixed length, same data type
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];
    numbers[0] = 0;//change a value
    println!("{:?}", numbers);//print the whole array
    println!("{}", numbers[0]);//print a single value
    let length = numbers.len();
    let storage = std::mem::size_of_val(&numbers);
    println!("{} bytes", storage);
    let slice: &[i32] = &numbers[0..2];
    println!("{:?}", slice);
}