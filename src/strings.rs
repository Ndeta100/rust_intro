//Primitives str=Immutable fixed-length string somewhere in memory
//Strings==growable, heap-allocated data structure -Use when you need to modify or own string data
pub fn run(){
    //not growable
let hello="hello world";
//growable
let mut str=String::from("hello world ");
str.push_str("world");
// Capacity in bytes
println!("capacity:{}", str.capacity());

// check if is empty
println!("Is empty:{}", str.is_empty());

// conatains
println!("Those it contains the word world:{}",str.contains("world"));
// Replace
println!("Replace: {}",str.replace("world","ndeta"));

// Looping through string by white space

for word in str.split_whitespace() {
println!("{}", word);
}
// create string with capacity
let mut s=String::with_capacity (10);
s.push('a');
s.push('b');
println!("{}",s);

// Assertion testing
assert_eq!(2,s.len());
assert_eq!(10,s.capacity());
// get length of string
println!("Length:{}", str.len());
}