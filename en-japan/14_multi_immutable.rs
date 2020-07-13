fn main () {
  let s = "this is str".to_string();
  
  let ref1 = &s;
  let ref2 = &s;

  println!("ref1: {}", ref1);
  println!("ref2: {}", ref2);
}