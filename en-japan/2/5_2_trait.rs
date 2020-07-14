trait Expand {
  fn say(&self);
}
impl Expand for i64 {
  fn say(&self){
    for n in 0..*self {
      println!("hey!({})", n);
    }
  }
}

fn main(){
  let num: i64 = 20;
  num.say();
}