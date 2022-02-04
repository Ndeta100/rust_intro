//Variables hold primitive data or references to data 
// Variables are immutable by default
// Rust is a block scoped language

pub fn run(){
let name="ndeta";
let mut age=67;
println!("My name is {} and I am {} years old", name, age);

age=22;

println!("My name is {} and I am {} years old", name, age);
}