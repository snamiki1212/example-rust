struct S;
trait Trait {
  fn say(&self);
}
impl Trait for S {
  fn say(&self) {
    println!("hey");
  }
}

fn main (){
  let s = S;
  say_say(s);

  // 同じ用にdummyがsay funcを持っていても
  // dummy はTraitの実装ではないので
  // トレイト境界で弾かれて、エラー
  // let d = Dummy;
  // say_say(d);
}

// <型パラメータ: トレイト名>
// <T: Trait>
// => 型パラメータ：変数的なの。
fn say_say<T: Trait>(s: T){
  s.say();
  s.say();
}


// struct Dummy;
// impl Dummy {
//   fn say(&self){
//     println!("it's dummy");
//   }
// }

