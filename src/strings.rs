/**
 * Primitive str = Immutable fixed-length string somewhere in memory
 * String = Growable, heap-allocated data structure - Use when you need to modify or own
*/

pub fn run() {
  let mut hello = String::from("Hello ");
  // length
  println!("Length: {}",hello.len());
  println!("{}",hello);
  // Push char
  hello.push('W');
  println!("{}",hello);
  // Push string
  hello.push_str("orld");
  println!("{}",hello);

  // Capacity in bytes
  println!("{}",hello.capacity());

  // Check is empty
  println!("{}",hello.is_empty());

  // Contains
  println!("{}",hello.contains("Worl"));

  // Replace
  println!("{}",hello.replace("World", "there"));

  // Loop through string by whitespace
  for word in hello.split_whitespace() {
    println!("{}", word);
  }

  // Create string with capacity
  let mut s = String::with_capacity(10);

  s.push('1');
  s.push('2');

  println!("{}", s);

  // Assertion testing
  assert_eq!(2,s.len());
  assert_eq!(10,s.capacity());

}