//13.1
// use g_game::{Summary, News, Tweet};
#[derive(Debug)]
struct Ww {
    w: i32,
    s: i32,
}


fn main() {
    let mut ws = [
        Ww{w: 5, s: 2},
        Ww{w: 3, s: 4},
        Ww{w: 1, s: 6},
    ];
    ws.sort_by_key(|s| s.w);
    
    println!("{:#?}", ws)
}
