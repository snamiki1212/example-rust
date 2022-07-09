use std::thread::spawn;

fn main(){
  let v = 10;
  let f = move || v * 2;

  let result = spawn(f).join();
  println!("result = {:?}", result);
  // 

  match spawn(|| panic!("Im panicked!")).join() {
    Ok(_) => println!("Success"),
    Err(maybe_str) => {
      let str = maybe_str.downcast_ref::<&str>();
      println!("[Catch] failed: {:?}", str);
    }
  }
}
