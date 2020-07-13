fn main () {
  let mut x = "this is msg".to_string();
  x = "new msg".to_string();

  let not_changable = &x;
  let changable = &mut x; // <== Error。immu / mut なrefの共存ができない。

  println!("not changable {}", not_changable);
  println!("changable {}", changable);
}