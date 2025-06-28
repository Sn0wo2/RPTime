mod config;
mod proxy;

use reqwest::Client;
use std::time::Duration;
use tokio::time::Interval;

#[tokio::main]
async fn main() {
    let cfg = config::settings::load_config();

    let mut global_tick: Option<Interval> = if cfg.interval > 0 {
        Some(tokio::time::interval(Duration::from_millis(cfg.interval)))
    } else {
        None
    };
    let mut per_url_tick: Option<Interval> = if cfg.url_interval > 0 {
        Some(tokio::time::interval(Duration::from_millis(cfg.url_interval)))
    } else {
        None
    };

    let mut client_builder = Client::builder();

    if let Some(proxy_url) = cfg.proxy {
        if !proxy_url.eq("direct") && let Ok(proxy) = proxy::builder::build_proxy(&proxy_url) {
            client_builder = client_builder.proxy(proxy);
        } else {
            client_builder = client_builder.no_proxy();
        }
    }

    let client = client_builder.build().expect("failed to build client");

    loop {
        if let Some(tick) = global_tick.as_mut() { tick.tick().await; }
        for url in &cfg.urls {
            if let Some(tick) = per_url_tick.as_mut() { tick.tick().await; }

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
