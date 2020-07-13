fn main () {
  let callback = add_one;
  let result = callback(10);
  println!("{}", result);
}

fn add_one(num: usize) -> usize {
  return num + 1;
}