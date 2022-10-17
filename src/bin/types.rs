pub fn run() {
    //Integers: u32- no negatives i32- normal integer
    //Floats: f32, f64
    let x = 1;//i32
    let y = 2.5;//f64
    let z: i64 = 1234;//i64
    let a: i32 = std::i32::MAX;
    println!("Max i32: {}", a);
    let bool1 = true;
    let ch = 'a';
    let face = '\u{1F600}';//using unicodes
    println!("face: {}", face);
}