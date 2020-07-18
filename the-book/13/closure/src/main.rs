fn main() {
    let simulated_user_specified_value = 10;
    let simulated_radom_number = 7;

    generate_workout(simulated_user_specified_value, simulated_radom_number)
}

use std::thread;
use std::time::Duration;
fn simulated_expensive_calculatoin(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            simulated_expensive_calculatoin(intensity)
        );

        println!(
            "Next, do {} situps!",
            simulated_expensive_calculatoin(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} miutes!",
                simulated_expensive_calculatoin(intensity)
            );
        }
    }
}
