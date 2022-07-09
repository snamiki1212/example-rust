use std::thread::spawn;

fn main() {
    my_fn1();
}


fn hello(){
    println!("hello!");
}

fn my_fn1(){
    let _ = spawn(hello).join();
    let h = || println!("my fn1");
    let _ = spawn(h).join();
}