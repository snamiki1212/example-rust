fn main () {
  let mut s = "this is str".to_string();
  println!("use var before mutation: {}", s);
  s = "new msg".to_string();
  
  let ref1 = &mut s;
  let ref2 = &mut s; // <== ここでError。mutationなrefは2つ以上借用できない

  println!("ref1: {}", ref1);
  println!("ref2: {}", ref2);
}