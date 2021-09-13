/**
 * Primitive Types--
 * Integers: u8, 18, u16, i16, u32, i32, i64, u64, u128, i128 (number of bits they take in memory)
 * Floats: f32, f64
 * Boolean (bool)
 * Characters (char)
 * Tuples
 * Arrays
 **/

// Rust is a statically typed language, which means that it must know 
// the types of all variables at comile time, however, the compiler can usally infer ehat type we want to use base on the value and how we use it

pub fn run() {
  // default is "i32"
  let _x = 1;

  // default is "f64"
  let _y = 2.4;

  // add explicit type
  let _z:i64 = 256538829191;

  // Find max size
  println!("Max i32: {}",std::i32::MAX);
  println!("Max i64: {}",std::i64::MAX);

  // Boolean
  let is_active = true;

  let is_greater = 10<5;

  // char
  let a = 'a';
  let b = '😁';

  println!("{:?}",(_x,_y,_z,is_active,is_greater,a,b));
}