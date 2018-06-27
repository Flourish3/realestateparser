extern crate parser;

use parser::scraper::*;

fn main() {
    match scrape_websites() {
        Ok(_s) => println!("Parsing went ok!"),
        Err(e) => panic!("Error scaping - Err: {}", e),
    };

}
