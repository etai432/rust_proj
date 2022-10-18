pub fn run() {
    //resizable array
    let mut numbers: Vec<i32> = vec![1,2,3,4];
    println!("the vector: {:?} \nthe first index: {} \nthe vector's length: {} \nthe vector occupies {} bytes", numbers, numbers[0], numbers.len(), std::mem::size_of_val(&numbers));

    numbers.push(5);//add elements
    numbers.push(6);
    numbers.pop();//remove last element
    println!("the vector: {:?}", numbers);

    for x in numbers.iter_mut() {
        *x += 1;
    }
    println!("the vector: {:?}", numbers);
}