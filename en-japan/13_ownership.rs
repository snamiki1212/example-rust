fn main () {
  let x = "this is msg".to_string();
  p(x);

  p(x); // << Error happen
}

fn p (n: String) {
  println!("n is {}", n);
}