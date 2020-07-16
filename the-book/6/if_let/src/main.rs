fn main() {

    let some_value = Some(0);


    if let Some(0) = some_value {
        println!("if-let");
    }

    if let Some(0) = some_value {
        println!("if-let-else:if");
    }else{
        println!("if-let-else:else");
    }

    match some_value {
        Some(0) => println!("match"),
        _ => (),
    }
}
