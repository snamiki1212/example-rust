fn main () {
  let list1: [isize; 3] = [1, 2, 3];
  let list1_slice = &list1; // slice

  // direct
  println!("list1: {:?}", list1);
  println!("list1_slice: {:?}", list1_slice);
  println!("------------");

  // for
  for i in list1_slice {
    println!("item: {}", i);
  }
  println!("------------");

  // index
  println!("idx: {}", list1[1]);
}