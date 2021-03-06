#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil
}

use List::{Cons, Nil};
use std::rc::Rc;

fn main(){ 
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(5, Rc::clone(&a));

    println!("b : {:?}", b);
    println!("c : {:?}", c);
}