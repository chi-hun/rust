//10.1
use std::io;
use rand::Rng;

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32)-> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        Guess {
            value: value,
        }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    let answer: i32 = rand::thread_rng().gen_range(1..100);

    loop {
        let mut g_input = String::new();
        io::stdin().read_line(&mut g_input).expect("fail");
        let g_input_i32: i32 = g_input.trim().parse().expect("fail");
        let guess_s: Guess = Guess::new(g_input_i32);
        let guess: i32 = guess_s.value();

        match guess.cmp(&answer) {
            std::cmp::Ordering::Less => println!("작습니다"),
            std::cmp::Ordering::Greater => println!("큽니다"),
            std::cmp::Ordering::Equal => {
                println!("정답입니다");
                break;
            }
        }
    }
}
