pub fn run() {
    let args: Vec<String> = std::env::args().collect();
    let command = &args[1..args.len()]; // the first arg (0 is the path)
    let name = "Brad";
    println!("Args: {:?}", args);
    println!("command: {:?}", command);

    if command[0] == "hello" {
        println!("hi {}, how are you? ", name)
    }
}