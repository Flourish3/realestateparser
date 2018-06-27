#[macro_use]
extern crate serde_derive;
extern crate toml;
extern crate hyper;

pub mod scraper;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}