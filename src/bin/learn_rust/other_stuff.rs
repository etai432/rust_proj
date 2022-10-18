// use std::io;
// use rand::Rng;
// use::std::ops::Add;
// use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::Write;
use std::io::BufReader;
use std::io::ErrorKind;

//generic function
// fn get_sum_gen<T: Add<Output = T>>(x: T, y: T) -> T {
//     return x + y;
// }

//modules
// mod pizza_order {
//     // To access the struct and the part to make public must both use pub
//     pub struct Pizza {
//         pub dough: String,
//         pub cheese: String,
//         pub topping: String,
//     }

//     // Implement functionality for the Pizza struct
//     impl Pizza {
//         pub fn lunch(topping: &str) -> Pizza {
//             Pizza {
//                 dough: String::from("regular dough"),
//                 cheese: String::from("mozzarella"),
//                 topping: String::from(topping),
//             }
//         }
//     }

//     // help_customer is public so functions can call it
//     pub mod help_customer {
//         // This function is private
//         fn seat_at_table() {
//             println!("Customer seated at table");
//         }

//         // Making help_customer public doesn't make this child
//         // function public so we must also make it public
//         pub fn take_order() {
//             seat_at_table();

//             // super allows me to access pizza in the parent scope
//             let cust_pizza: super::Pizza =
//                 super::Pizza::lunch("veggies");

//             serve_customer(cust_pizza);
//         }

//         fn serve_customer(cust_pizza: super::Pizza){
//             println!("The customer is served a regular pizza with {}", cust_pizza.topping);
//         }
// // This is the public function that allows our other file access

//     }
// }
// pub fn order_food() {
//     crate::other_stuff::pizza_order::help_customer::take_order();
// }
pub fn run() {
    //input
    // let mut name = String::new();
    // io::stdin().read_line(&mut name).expect("didnt receive input");
    // println!("{}", name);

    //random
    // let mut random_num = rand::thread_rng().gen_range(1..101);
    // println!("{}", random_num);

    //casting
    // let int_u8: u8 = 5;
    // let int2_u32: u32 = int_u8 as u32;
    // println!("{}", int2_u32);

    //use generic function
    // println!("{}", get_sum_gen(1, 1));

    //ownership
    //Stack- Stores values in a "last in first out" data on the stack must have a defined fixed size
    //Heap- when putting data on the heap you request a certain amount of space. the OS finds space available and returns an address for that space called a pointer.
    //Rules:
        //1. Each value has a variable that's called its owner
        //2. There is only one owner at a time
        //3. when the owner goes out of scope the value disappears
    

    //Hashmaps
    // let mut heroes = HashMap::new();
    // heroes.insert("superman", "Clark Kent");
    // heroes.insert("Batman", "Bruce Wayne");
    // heroes.insert("The Flash", "Barry Allen");

    // for (k, v) in heroes.iter() {
    //     println!("{} = {}", k, v);
    // }
    // println!("Length: {}", heroes.len());

    // if heroes.contains_key(&"Batman") {
    //     let the_batman = heroes.get(&"Batman");
    //     match the_batman {
    //         Some(x) => println!("Batman is a hero"),
    //         None => println!("Batman is not a hero"),
    //     };
    // }

    //error handling & files
    //panic!()- kills the program lol
    // let path = "lines.txt";
    // let output = File::create(path);
    // let mut output = match output {
    //     Ok(file) => file,
    //     Err(error) => panic!("error: {:?}", error)
    // };
    // write!(output, "just some random text ig").expect("Failed to write to the file");
    // let input = File::open(path).unwrap();
    // let buffered = BufReader::new(input);
    // for line in buffered.lines() {
    //     println!("{}", line.unwrap());
    // }

    // let output2 = File::create("rand.txt");
    // let output2 = match output2 {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("rand.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("cant create file"),
    //         },
    //         _other_error => panic!("problem opening file"),
    //     },
    // };



}