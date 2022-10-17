pub fn run() {
    println!("hello from print.rs");
    //basic formating:
    println!("number: {}, string: {}", 1, "hi");

    //Positional Arguments
    println!("{0} is from {1} and {0} likes to {2}", "Brad", "Mass", "code");

    //named arguments
    println!("{name} likes to play {activity}", name = "Jhon", activity = "Baseball");

    //placeholder traits
    println!("100 in Binary: {:b}", 100);

    //Placeholder for debug trait
    println!("{:?}", (12, true, "hi"))
}