use std::fs;
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let info: String = fs::read_to_string(config.file_path)?;
    println!("info : \n{}", info);
    Ok(())
}

pub struct Config {
    pub query : String,
    pub file_path : String,
}

impl Config {
    pub fn new(arg: &[String]) -> Result<Config, &'static str> {
        if arg.len() < 3 {
            return Err("not enough arguments");
        }

        let query = arg[1].clone();
        let file_path = arg[2].clone();
        Ok(Config {query, file_path})
    }
}
