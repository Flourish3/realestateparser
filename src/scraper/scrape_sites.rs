use scraper::types::*;
use hyper::rt::{self, Future, Stream};
use hyper::Client;
use std::io::{self, Write};

pub fn scrape_sites(config : &Config ) -> Result<bool,String> {
    let client = Client::new();

    let uri = "http://httpbin.org/ip".parse().unwrap();

    println!("Making request");
    rt::run(
        client
            .get(uri)
            .and_then(|res| {
                println!("Response {}", res.status());
                res.into_body().for_each(|chunk| {
                    io::stdout()
                        .write_all(&chunk)
                        .map_err(|e| panic!("Example expects stdout is open, error={}", e))
                })
            })
            .map_err(|err| println!("Error: {}", err)),
    );

    Ok(true)
}