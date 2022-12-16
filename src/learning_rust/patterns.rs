pub fn run() {
    let (x, y, z) = (1, 2, 3);
    println!("{}", x);

    //dont do:
    // let x: Option<&str> = None;
    // let Some(x) = x;
    // if let x = 5 {
    //     println!("{}", x);
    // }
}