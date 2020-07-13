fn main () {
  match 8 {
    0 => println!("this is 0"),
    n @ 1..=10 => println!("this is num {}", n),
    _ => println!("more than 10")
  }
}