fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "the are of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

struct Rectangle {
    width: u32,
    height: u32,
}

fn area(dimensions: &Rectangle) -> u32 {
    dimensions.width * dimensions.height
}