use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let list = vec![
            String::from("no1"),
            String::from("no2"),
            String::from("no3"),
            String::from("no4"),
            String::from("no5"),
        ];
        for item in list {
            tx.send(item).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    for received in rx {
        println!("received: {}", received);
    }
}
