pub fn run() {
  greeting("hello","joe");

  // Bind function values to variables
  let get_sum = add(2,9);
  println!("Sum: {}", get_sum);

  // Closur
  let add_nums = |n1:i32,n2:i32| n1+n2;
  println!("Sum: {}", add_nums(2,4));
}

fn greeting(greet: &str, name:&str){
  println!("{} {}, nice to meet you", greet, name );
}

fn add(n1: i32,n2: i32)-> i32 {
  // ? If no semi colon it will return
  n1 + n2
}