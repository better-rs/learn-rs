use reqwest;
use std::collections::HashMap;

fn rs_http_get_text(url: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    let resp = client.get(url).header("User-Agent", "Firefox/47.0").send()?;
    Ok(resp.text()?)
}

fn rs_http_get_json(url: &str) -> Result<HashMap<String, String>, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    let resp = client.get(url).header("User-Agent", "Firefox/47.0").send()?;
    Ok(resp.json()?)
}

#[cfg(test)]
mod tests {
    use crate::http::{rs_http_get_json, rs_http_get_text};

    #[test]
    fn test_http_get() {
        let url = "https://api.github.com/zen";
        let result = rs_http_get_text(url);
        println!("{:?}", result);
    }

    #[test]
    fn test_http_get_json() {
        let url = "https://api.github.com/orgs/rust-lang";
        let result = rs_http_get_text(url);
        println!("{:?}", result);
    }
}
