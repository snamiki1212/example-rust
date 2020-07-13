fn main () {
  // 1st
  let x: i32 = 1 + 9;
  println!("{}", x);
  println!("-----");

  // 2nd
  let mut y = 21;
  println!("{}", y);
  y = 22;
  println!("{}", y);
  println!("-----");

  // 3rd
  let z: i32 = 30;
  println!("{}", z);
  let z: i32 = 31;
  println!("{}", z);
  let z: &str = "abc";
  println!("{}", z);
  println!("-----");

}