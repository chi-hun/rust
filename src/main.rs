#[derive(Debug)]
struct psize {
    width: u32,
    height: u32,
}


fn main() {
    let size_wh = psize{
        width : 40,
        height : 150,
    };
    dbg!(&size_wh);
    let area = ca_size(&size_wh);
    println!("area : {}", area);
    println!("haha");

}

fn ca_size(wh: &psize) -> u32 {
    wh.width * wh.height
}