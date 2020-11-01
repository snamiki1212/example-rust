fn main() {
    {
        let c = CustomSmartPointer { data: String::from("My studd" )};
        let d = CustomSmartPointer { data: String::from("other studd" )};
        println!("created and c is {:?}", c);
    }
    println!("next");

    // -
    // let m = MyBox::new(String::from("Rust"));
    // hello(&m);
    // --

    // let x = 5;
    // let y = MyBox::new(x);

    // assert_eq!(5, x);
    // assert_eq!(5, *y);
}

#[derive(Debug)]
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self){
        println!("Dropping data {}", self.data);
    }
}

fn hello(name: &str) {
    println!("Hello, {}", name);
}

use std::ops::Deref;
struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        println!("deref!!");
        &self.0
    }
}