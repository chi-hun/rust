//8.2
// #[derive(Debug)]
use std::fs::File;

fn main() {
    let file_load_result = File::open("input.txt").expect("fuck");
}
