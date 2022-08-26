#[allow(dead_code)]

// primitize str = immutable & fixed length
// String = growable, can be modified
// * important

pub fn run() {
  let mut hello = String::from("hello");

  // push char
  hello.push(' ');

  // push string
  hello.push_str("world");

  // loop through string by whitespace 
  for word in hello.split_whitespace() {
    println!("{}", word)
  }

  // create string with capacity
  let mut frog = String::with_capacity(4);
  frog.push_str("frog");

  assert_eq!(4, frog.len());

  println!("{} | length: {}", hello, hello.len());
  // capacity (in bytes), is_empty, contains 'world'?
  println!("capacity: {} | is_empty: {} | contains 'world': {}", hello.capacity(), hello.is_empty(), hello.contains("world"));
}