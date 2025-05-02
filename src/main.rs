#[derive(Debug)]
struct Psize {
    width: u32,
    height: u32,
}

impl Psize {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn scope(&self, sc: u32) -> u32 {
        self.width * self.height * sc
    }
}

fn main() {
    let size_wh = Psize{
        width : 40,
        height : 150,
    };
    dbg!(&size_wh);
    println!("area : {}", size_wh.area());
    println!("scope : {}", size_wh.scope(2));
}