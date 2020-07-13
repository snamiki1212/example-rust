fn main() {
  println!("{}", square_sum(10));
}

fn square_sum(num: isize) -> isize {
  (0..num)
    .filter(|i| i % 2 ==0)
    .map(|i| i*i)
    .sum()
}

