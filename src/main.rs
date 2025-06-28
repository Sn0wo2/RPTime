mod config;
mod proxy;

use reqwest::Client;
use std::time::Duration;
use tokio::time::interval;


#[tokio::main]
async fn main() {
    let cfg = config::settings::load_config();
    let mut interval = interval(Duration::from_secs(5));

    let mut client_builder = Client::builder();

    if let Some(proxy_url) = cfg.proxy {
        if let Ok(proxy) = proxy::builder::build_proxy(&proxy_url) {
            client_builder = client_builder.proxy(proxy);
        }
    }
    
    let client = client_builder.build().expect("failed to build client");

    loop {
        interval.tick().await;

        for url in &cfg.urls {
            match client.get(url).send().await {
                Ok(resp) if resp.status().is_success() => {
                    println!("{} => SUCCESS", url);
                }
                Ok(resp) => {
                    println!("{} => error: HTTP {}", url, resp.status());
                }
                Err(err) => {
                    println!("{} => error: {}", url, err);
                }
            }
        }
    }
}
