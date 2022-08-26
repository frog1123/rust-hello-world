#[allow(dead_code)]

// info on string in strings.rs
pub fn run() {
  let _num: i32 = 1123;

  // find max size
  println!("max size (i32): {}", std::i32::MAX);

  let boolean: bool = true;
  let bool_from_expression: bool = 0 > 1123;
  
  // must be only 1 character & single quotes ('')
  let character: char = 'a';
  let emogi: char = '\u{1F334}';

  println!("{:?}", (boolean, bool_from_expression, character, emogi));
}