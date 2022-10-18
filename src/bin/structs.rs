//traditional struct
struct Color {
    red: u8,
    green: u8,
    blue: u8
}
//Tuple struct
struct Color1(u8, u8, u8);

//struct with functions
struct Person {
    first_name: String,
    last_name: String
}
//add methods
impl Person {
    //construct
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }
    //return full name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
    //change last name
    fn set_last(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    //name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }


pub fn run() {
    //Structs - used to create custom data types
    let mut c = Color {
        red: 255,
        blue: 0,
        green: 0
    };
    println!("Color: {} {} {}", c.red, c.green, c.blue);
    c.red = 200;
    println!("Color: {} {} {}", c.red, c.green, c.blue);
    //using tuple struct
    let mut c1 = Color1(255, 0, 0);
    println!("Color: {} {} {}", c1.0, c1.1, c1.2);
    c1.0 = 200;
    println!("Color: {} {} {}", c1.0, c1.1, c1.2);

    let mut p1 = Person::new("John", "Doe");
    println!("Person: {} {}", p1.first_name, p1.last_name);
    p1.set_last("Williams");
    println!("full name after change: {}", p1.full_name());
    println!("name as tuple: {:?}", p1.to_tuple())


}