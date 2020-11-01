use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);

    // 1st channel
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

    thread::sleep(Duration::from_millis(500));

    // 2nd channel
    thread::spawn(move || {
        let list = vec![
            String::from("no1_2"),
            String::from("no2_2"),
            String::from("no3_2"),
            String::from("no4_2"),
            String::from("no5_2"),
        ];
        for item in list {
            tx1.send(item).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("received: {}", received);
    }
}
