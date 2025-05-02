use std::time::Instant;

fn main() {
    let start = Instant::now();
    let mut a: u128 = 1;
    let mut b: u128 = 1;
    let mut c: u128 = 0;
    
    for _ in 0..90 {
        c = a + b;
        a = b;
        b = c;
    }
    let duration = start.elapsed();
    println!("{c}");
    println!("Elapsed: {:?}", duration);
}

//1.96µs