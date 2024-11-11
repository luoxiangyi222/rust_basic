fn main() {

    // let mut a = 10;
    // a = 5;
    
    // println!("Hello, world!, {}", a);


    let arr = [0, 1, 2];  // point to memory
    let slice = &arr[1 .. 3];

    println!("{:?}", slice)
}
