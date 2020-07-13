fn main () {
  let x = "this is msg".to_string();

  let x_ref = &x;

  let y = x; // <== ERror。ここでxがdropして、所有権がyに映る。だけども、x_refがxを参照しているので、エラーになる

  println!("{}, {}", x_ref, y)
}