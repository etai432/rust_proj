// use std::io;
// use rand::Rng;
// use::std::ops::Add;
// use std::collections::HashMap;

//generic function
// fn get_sum_gen<T: Add<Output = T>>(x: T, y: T) -> T {
//     return x + y;
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

}