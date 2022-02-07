pub fn run(){
    let mut number:Vec<i32> =vec![1,2,3,4,5];
    // reassign a value 
    number[2]=20;
    // add onto vector length
    number.push(3);
    number.push(50);

    // Pop of last value
    number.pop();
    println!("{:?}", number);
    // Get single val
    println!("single value {}", number[0]);
    //get vector length
    println!("vector length {}", number.len());

    // vectors are stack allocated
    println!("this vector occupies {} bytes", std::mem::size_of_val(&number));

    //Get slice 
    let slice: &[i32]=&number[0..2];
    println!("slice {:?}", slice);

    // loop through the vector values and
    for x in number.iter() {
        println!("Number {}", x);
    }
    // loop and mutate values
    for x in number.iter_mut() {
       *x *=2;
    }
    println!("Number vec: {:?}", number);
}