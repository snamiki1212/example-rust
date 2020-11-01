fn main() {
    let list = Cons(1,
        Box::new(Cons(2,
            Box::new(Cons(3,
                Box::new(Nil))))));

    println!("b = {}", b);
}

use List::{Cons, Nil};

enum List{
    Cons(i32, Box<List>),
    Nil,
}