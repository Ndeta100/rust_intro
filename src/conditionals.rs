// It is use to check the condition of something and then act on the results

pub fn run(){
let age=12;
let check_id=false;
let know_person_of_age=true;
//If/else
if age>=21 && check_id || know_person_of_age{
    println!("Bartender said what will you want to drink");
}else if age<=21 && check_id{
    println!("Sorry you have to leave");
}else{
    println!("I will need to see your id");
}
}