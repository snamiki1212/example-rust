trait Animale {
  // interface
  fn say(&self);
  
  // 実装もかける
  fn walk(&self) {
    println!("walking...");
  }
}

struct Duck; // 空のUnit構造体
impl Animale for Duck {
  fn say(&self) {
    println!("quack");
  }
}

struct Cat;
impl Animale for Cat {
  fn say(&self) {
    println!("mew");
  }
}

fn main (){
  let cat = Cat;
  let duck = Duck;
  
  cat.say();
  duck.say();
}

