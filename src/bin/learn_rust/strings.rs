pub fn run() {
    let hi = "hi";//primitive str = immutable fixed length string somewhere in memory
    let mut hello = String::from("hello");//String = Growable, heap-allocated data structure - Use when you need to modify or own string data
    let length = hello.len();//get length of a string
    hello.push(' ');//for characters
    hello.push_str("world");
    let capacity = hello.capacity();
    let is_empty = hello.is_empty();
    let does_contain = hello.contains("world");
    hello = hello.replace("world", "there");
    println!("{}", hello);
    
    //loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }
    
    let mut s = String::with_capacity(10);//making a string with capacity
    s.push('a');
    s.push('b');

    //Assertion testing
    assert_eq!(10, s.capacity())

}