pub fn run(){
  let age=18;
  let check_id = false;
  if age >=21{
    println!("Bartender: What would like to drink?");
  } else if age<21 && check_id{
    println!("Bartender: You have to leave");
  }else{
    println!("Baetender: I have to check your id");
  }

  // Shorthand if
  let is_of_age = if age>=21 {true} else {false};
  println!("Is of age: {}", is_of_age);
}