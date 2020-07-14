
fn main () {
  let r1 = pair(1, 2);
  println!("{}, {}", r1.0, r1.1);

  // turbofish syntax
  let r2 = pair::<isize, isize>(10, 20);
  println!("{}, {}", r2.0, r2.1);
}

fn pair<T, S>(t: T, s: S) -> (T, S) { (t, s) }