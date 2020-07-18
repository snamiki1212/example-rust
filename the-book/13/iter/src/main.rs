fn main() {
    println!("Hello, world!");
}

#[test]
fn iterator_demonstration(){
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));

}


#[test]
fn iterator_sum(){
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);
}

#[test]
fn non_consume() {
    let list =vec![1, 2, 3];
    let result: Vec<_> = list.iter().map(|item| item + 1).map(|item| item + 1).collect();
    println!("result {:?}", result);
}