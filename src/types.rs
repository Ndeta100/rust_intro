/*
Primitives types-----------
Integers:u8, u16, i16, i32, i64, u32,u128,(Numbers of bits they take in memory)
Floats: f32, f64
Boolean:(bool)
Characters(char)
Tuples
Arrays

*/

// Rust is a statically typed language, which means that it must know the types of all variables at compile time, however, the compiler can usually infer what type we want to use based on the value and how we use it
// use std::i32;
// use std::i64;
pub fn run(){
//default is i32,
let x=1;

//default is f64
let y=3.5;

//add explicit type 
let z: i64=456789876543456789;

//find max size 
println!("Max i32: {}", std::i32::MAX);
println!("Max i64: {}", std::i64::MAX);

// Boolean
let is_active: bool = true;
println!("{:?}",(x,y,z ,is_active));

// get a boolean from an extension 
let is_greater: bool=10<5;

// characters
let a1='a'; 
let face='\u{1F600}';
println!("{:?}",(x,y,z ,is_greater,a1,face));

}