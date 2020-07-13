fn main () {
  // to_string = &str -> String
  let large: String = "abc".to_string();
  println!("{}", large);

  // as_str = String -> &str
  let mut small: &str = large.as_str();
  println!("{}", small);
}