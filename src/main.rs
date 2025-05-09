//10.1
use g_game::{Summary, News, Tweet};

fn main() {
    let news1 = News{
        title : String::from("war"),
        main : String::from("war is coming?"),
        who : String::from("faker"),
    };
    println!("{}", news1.sumarize());

    let tweet1 = Tweet{
        title : String::from("v-line"),
        hash : String::from("#face"),
        who : String::from("faker"),
    };
    println!("{}", tweet1.sumarize());
}
