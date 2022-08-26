#[allow(dead_code)]

pub fn run() {
  let name = "kevin";
  let friend = "billy";

  // vars are immutable be default
  // mut makes it mutable
  let mut age = 20;
  age = 21;

  println!("name: {}, friend(s): {}, age: {}", name, friend, age);

  // define const
  // usually in uppercase
  const COOL: i32 = 1123;
  println!("cool: {cool}", cool = COOL);

  // assign multiple vars
  let (my_name, my_age) = ("joe", 25);
  println!("{name} is {age}", name = my_name, age = my_age);
}