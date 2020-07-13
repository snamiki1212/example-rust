fn main() {
  fizzbuzz(20);
}

fn fizzbuzz(num: usize) {
  for i in 0..num {
    if i % 15 == 0 {
      println!("FizzBuzz");
    }
    else if i % 3 == 0 {
      println!("Fizz");
    }
    else if i % 5 == 0 {
      println!("Buzz");
    }
    println!("{}", i);
  }
}
