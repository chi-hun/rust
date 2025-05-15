//16.1
// use g_game::{Summary, News, Tweet};
use std::thread;
use std::time::Duration;

fn main() {
    let side_1 =thread::spawn(|| {
        for i in 1..10 {
            println!("num : {} for side_1 thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });


    for i in 1..5 {
        println!("num : {} for main thread", i);
        thread::sleep(Duration::from_millis(1));
    }

    side_1.join().unwrap();
}
