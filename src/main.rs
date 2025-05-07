//8.2
// #[derive(Debug)]
fn main() {
    let old_ss = "old";
    let ss: String = String::new();
    let mut ss = old_ss.to_string(); 
    ss.push_str("_new");
    println!("{}",ss);
}