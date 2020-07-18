use std::fs::File;
// use std::io::ErrorKind;
use std::io;
use std::io::Read;

fn main() {
    match read_username_from_file() {
        Ok(v) => println!("ok {}", v),
        Err(e) => println!("err {}", e),
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    // let mut f = match File::open("hello.txt") {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };

    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}