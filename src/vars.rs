// * variables hold primitive data or references to data
// * variables are immutable by default
// * Rust is a block-scoped language

pub fn run() {
  let name = "foo";
  let mut age = 23;
  println!("My name is {} and I am {}",name, age);
  
  age = 24;
  
  println!("My name is {} and I am {}",name, age);

  // Define const
  const ID: i32 = 001;
  println!("ID: {}",ID);

  // Assign multiple vars
  let ( my_name, my_age ) = ("bar",23);

  println!("{} is {}",my_name,my_age);
}