// Arrays - Fixed list where elements are the same data types
use std::mem;

pub fn run() {
  let mut numbers: [i32;5] = [1,2,3,4,5];

  println!("{:?}", numbers);

  // single value
  println!("{}", numbers[0]);

  // Reassigned values
  numbers[2]= 60;
  println!("{}", numbers[2]);

  // length
  println!("{}", numbers.len());

  // Arrays are stack allocated
  println!("Array occupies {} bytes", mem::size_of_val(&numbers));

  // Get slice
  let slice: &[i32] = &numbers[2..5];

  println!("{:?}", slice);
}