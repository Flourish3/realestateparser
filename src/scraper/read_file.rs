use scraper::types::*;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use toml;

pub fn parse_sites_from_file(file_name : String) -> Result<Config, String> {
    // get Current Directory
    let cwd: String = match env::current_dir() {
        Ok(c) => c.display().to_string(),
        Err(e) => return Err(e.to_string()),
    };

    println!("Current Working Directory: {}", cwd);

    let sites_path = cwd + "\\" + &file_name;

    println!("File path: {}", sites_path);

    let mut toml_file = match File::open(sites_path) {
        Ok(f) => f,
        Err(e) => return Err(e.to_string()),
    };

    let mut toml_string = String::new();

    match toml_file.read_to_string(&mut toml_string) {
        Ok(s) => s,
        Err(e) => return Err(e.to_string()),
    };

    println!("Toml string: {}", toml_string);

    let config: Config = match toml::from_str(&toml_string) {
        Ok(t) => t,
        Err(e) => return Err(e.to_string()),
    };

    Ok(config)
}
