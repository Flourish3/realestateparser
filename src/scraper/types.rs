#[derive(Deserialize)]
pub struct Config {
    websites: Websites,
}

#[derive(Deserialize)]
struct Websites {
    list: Vec<String>,
}
