#[macro_use]
macro_rules! items {
    ( $($x:expr),*) => {
        let mut num = 1;
        $(
            println!("{} is item number {}", $x, num);
            num += 1;
        )*
    };
}

pub fn run() {
    items![2, 5, 7, "hello"];
}