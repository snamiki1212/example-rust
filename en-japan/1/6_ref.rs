fn main () {
  // immutable
  let x = 1;
  let r = &x; // ref取得
  println!("{}", r);

  // muttable
  let mut a = 1;
  let b = &mut a; // ref取得
  *b = 2; // mutなら更新も可能
  println!("{}", *b);
}