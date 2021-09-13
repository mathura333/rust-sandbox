pub fn run() {
  // * Print to console
  println!("Hello from print.rs file");

  // * Basic formatting
  println!("{} is from {}","foo","bar");

  // * Positional Arguments
  println!("{0} is from {1} and {0} likes to {2}","foo","bar","baar");

  // * Named arguments
  println!("{name} like to play {game}",name="foo",game="bar");

  // * Placeholder traits
  println!("Binary: {:b} Hex: {:x} Octal: {:o}",10,10,10);

  // * Placeholder for debug trait
  println!("{:?}",(12,true, "hello"));

  // * Basic operation
  println!("10 + 10 = {}",10+10);
}