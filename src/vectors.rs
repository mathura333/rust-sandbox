// Vectors - Resizable arrays
use std::mem;

pub fn run() {
  let mut numbers: Vec<i32> = vec![1,2,3,4,5];

  println!("{:?}", numbers);

  // single value
  println!("{}", numbers[0]);

  // Reassigned values
  numbers[2]= 60;
  println!("{}", numbers[2]);

  // Add on to vector
  numbers.push(7);
  println!("{:?}", numbers);
  numbers.pop();
  // length
  println!("{}", numbers.len());

  // Vectors are stack allocated
  println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

  // Get slice
  let slice: &[i32] = &numbers[2..5];

  println!("{:?}", slice);

  for x in numbers.iter() {
    println!("{}", x);
  }

  for x in numbers.iter_mut() {
    *x *= 3;
  }

  println!("{:?}", numbers);
}