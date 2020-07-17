fn main() {
    let list = vec![34, 50, 25, 100, 65];
    let largest = largest(&list);

    println!("largest: {}", largest);
}


fn largest<T: std::cmp::PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}