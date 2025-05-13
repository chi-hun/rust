//12.6
// use g_game::{Summary, News, Tweet};
use std::env;
use g_game::Config;


fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });
    println!("{}", config.query);
    println!("{}", config.file_path);
    if let Err(e) = g_game::run(config) {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}
