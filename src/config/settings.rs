use std::fs;

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    #[serde(default)]
    pub proxy: Option<String>,

    pub urls: Vec<String>,

    #[serde(default = "default_interval")]
    pub interval: u64,

    #[serde(default = "default_url_interval")]
    pub url_interval: u64,
}

fn default_interval() -> u64 {
    5
}

fn default_url_interval() -> u64 {
    0
}

pub fn load_config() -> Settings {
    let settings = fs::read_to_string("config.yml").expect("Unable to read config file");
    serde_yaml::from_str(&settings).expect("Unable to parse config file")
}
