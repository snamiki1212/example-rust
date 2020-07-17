fn main() {
    let list = vec![34, 50, 25, 100, 65];

    let mut largest = list[0];
    
    for number in list {
        if number > largest {
            largest = number;
        }
    }

    println!("largest: {}", largest);
}
