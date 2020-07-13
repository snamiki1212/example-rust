fn main () {
  let r = func1();
  println!("{} / {} / {}", r.0, r.1, r.2);
}

fn func1 () -> (String, String, String) {
  return("a".to_string(), "b".to_string(), "c".to_string());
}