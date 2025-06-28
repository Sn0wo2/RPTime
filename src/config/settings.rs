use std::fs;

#[derive(serde::Deserialize)]
pub struct Settings {
    pub proxy: Option<String>,
    pub urls: Vec<String>,
}


pub fn load_config() -> Settings {
    let settings = fs::read_to_string("config.yml").expect("Unable to read config file");
    serde_yaml::from_str(&settings).expect("Unable to parse config file")
}