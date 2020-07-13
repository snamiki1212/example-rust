fn main () {
  let num = 1;

  // 数値はCopy型なので、ライフサイクルでdropしない。
  let n1 = num;
  let n2 = num;
  let n3 = num;

  println!("{}, {}, {}, {}", n1, n2, n3, num);
}