pub fn run(){
    let mut number:[i32; 5] =[1,2,3,4,5];
    // reassign a value 
    number[2]=20;
    println!("{:?}", number);
    // Get single val
    println!("single value {}", number[0]);
    //get array length
    println!("array length {}", number.len());

    // Arrays are stack allocated
    println!("this array occupies {} bytes", std::mem::size_of_val(&number));

    //Get slice 
    let slice: &[i32]=&number[0..2];
    println!("slice {:?}", slice);
}