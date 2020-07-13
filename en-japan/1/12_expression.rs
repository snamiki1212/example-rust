fn main () {
  let str = if true {
    "ok"
  }else if false {
    "ng"
  }else{
    "unknown"
  };
  println!("string is {}", str);

}