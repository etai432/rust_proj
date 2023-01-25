use core::any::Any; 

// pub fn run() {

// }

// pub fn collect_tasks() -> Vec<Box<dyn Fn()>> {

// }

pub fn build_vec() -> Vec<Box<dyn Any>> {
    let mut vec = Vec::new();
    vec.push(Box::new(|x: i32| x + 1) as Box<dyn Any>);
    vec.push(Box::new(|x: &str| x.len()) as Box<dyn Any>);
    vec.push(Box::new(|x: (i32, i32)| (x.0 + x.1, x.0 * x.1)) as Box<dyn Any>);
    println!("{:?}", vec[0]);
    for f in vec.iter() {
        if let Some(func) = f.downcast_ref::<fn(i32) -> i32>() {
            println!("i32: {}", func(2));
        } else if let Some(func) = f.downcast_ref::<fn(&str) -> usize>() {
            println!("string: {}", func("hello"));
        } else if let Some(func) = f.downcast_ref::<fn((i32, f32)) -> (f32, i32)>() {
            println!("tuple: {:?}", func((2, 3.5)));
        }
    }
    vec
}