extern crate oop;
use oop::{Draw, Screen, Button};

struct SelectBox {
    width:u32,
    height: u32,
    options: Vec<String>
}

impl Draw for SelectBox {
    fn draw(&self) {
        // 
    }
}

fn main(){ 
    let screen = Screen {
        components: vec![
            // 
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    Strinf::from("y"),
                    Strinf::from(":D"),
                    Strinf::from("n"),
                ]
            }),
            // 
            Box::new(Button {
                height: 50,
                width: 10,
                label: String::from("OK"),
            }),
        ],
    };
    screen.run();
}