use self::read_file::*;
use self::types::*;
use self::scrape_sites::*;

mod read_file;
mod types;
mod scrape_sites;

pub fn scrape_websites() -> Result<bool, String> {
    let config: Config = match parse_sites_from_file("sites.toml".to_string()) {
        Ok(t) => t,
        Err(e) => return Err(e.to_string()),
    };

    match scrape_sites(&config) {
        Ok(s) => s,
        Err(e) => return Err(e.to_string()),
    };

    Ok(true)
}
