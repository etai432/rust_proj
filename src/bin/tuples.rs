pub fn run() {
    //tuples are group together values of diff types
    //Max 12 elements
    let person: (&str, &str, i8) = ("Brad", "Mass", 37);
    println!("{} is from {} and is {}", person.0, person.1, person.2);
}