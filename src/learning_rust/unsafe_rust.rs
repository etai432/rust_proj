pub fn run() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
    }

    unsafe fn dangerous() {}
    
    unsafe {
        dangerous();
    }

    unsafe {
        println!("{}", abs(3))
    }
}
extern "C" {
    fn abs(input: i32) -> i32;
}