//Primitives str=Immutable fixed-length string somewhere in memory
//Strings==growable, heap-allocated data structure -Use when you need to modify or own string data
pub fn run(){
let hello="hello world";
let mut str=String::from("hello world ");
str.push_str("world");
// get length of string
println!("Length:{}", str.len());
}