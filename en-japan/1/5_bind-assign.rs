fn main () {
  // 束縛
  rebind();

  // 代入
  reassign();
}

fn rebind () {
  println!(">> start rebind.");
  let x: usize = 0;
  for i in 0..5 {
    let x: usize = i;
    println!("inner: {}", x)
  }
  println!("outer: {}", x);
}

fn reassign () {
  println!(">> start reassign.");
  let mut x: usize = 0;
  for i in 0..5 {
    x = x + i;
    println!("inner: {}", x);
  }
  println!("outer: {}", x);
}