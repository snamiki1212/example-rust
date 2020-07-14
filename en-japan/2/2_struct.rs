fn main () {
  // tupple
  let p = Point(1, 2);
  println!("point({},{})", p.0, p.1);

  // normal
  let u = User { name: "OK".to_string(), age: 10 };
  println!("name:{} / age:{}", u.name, u.age);
}

struct User {
  name: String,
  age: usize,
}

struct Point(usize, usize);
