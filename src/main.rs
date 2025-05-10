//11.1
// use g_game::{Summary, News, Tweet};

fn main() {
    let first_str = String::from("hello dd");
    let r;
    {
        let second_str = "what";
        let r = llwhat(&first_str.as_str(), &second_str);
    }
    

    fn llwhat<'a>(f: &'a str, s: &'a str) -> &'a str {
        if f.len() > s.len() {
            f
        } else {
            s
        }

    }

    println!("{}", r);
}
