pub fn run() {
    //refrence pointers- point to a resource in memory
    let arr1 = [1, 2, 3];
    let arr2 = arr1;
    println!("{:?}", (arr1, arr2));
    //must use & when pointing to non-primitive values
    let vec1: Vec<i32> = vec![1, 2, 3];
    let vec2 = &vec1;
    println!("{:?}", (&vec1, vec2));
}