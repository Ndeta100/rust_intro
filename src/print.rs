pub fn run(){
    //Print to console
    println!("Hello from the print rs file");

    // Basic formatting
    println!("Number: {}", 1);
    println!("{} is from  {}", "Ndeta", "Mars");

    // Positional Arguments
    println!("{0} is from {1} and {0} likes to {2}", "John", "Mars", "code");

    // Named arguments
    println!("{name} likes to play {activity}", name="Ndeta", activity="Football")
}