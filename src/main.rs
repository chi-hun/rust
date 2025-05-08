//8.2
// #[derive(Debug)]
fn main() {
    let s1 = String::from("안녕");
    let s2 = String::from(" 하세요");
    let s3 = s1 + &s2;
    println!("{}", s3);
    println!("{}", &s3[0..3]);
    for i in s3.chars() {
        println!("{}", i);
    }
    println!("{}", s3);
}
