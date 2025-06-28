use reqwest::{Proxy, Url};

pub(crate) fn build_proxy(proxy_url: &str) -> Result<Proxy, Box<dyn std::error::Error>> {
    let url_str = if !proxy_url.starts_with("http://") && !proxy_url.starts_with("https://") {
        format!("http://{}", proxy_url)
    } else {
        proxy_url.to_string()
    };

    let parsed = Url::parse(&url_str)?;

    let url_with_port = if parsed.port().is_none() {
        let mut url = parsed.clone();
        url.set_port(Some(match url.scheme() {
            "http" => 80,
            "https" => 443,
            _ => 8080,
        })).unwrap();
        url.to_string()
    } else {
        url_str
    };

    let proxy = Proxy::all(url_with_port)?;

    Ok(if !parsed.username().is_empty() {
        let password = parsed.password().unwrap_or("");
        proxy.basic_auth(parsed.username(), password)
    } else {
        proxy
    })
}