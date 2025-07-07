use reqwest::{blocking, header::{HeaderValue, AUTHORIZATION}};
use serde_json::Value;

const AUTHORIZATION_TOKEN: &'static str = "y0__xC0qIb_Axi85zgg4sDE4BOBkr9aha6pKslrohTx-3ivfnxOHw";

fn main() -> anyhow::Result<()> {
    let client = blocking::Client::new();
    let auth_header = HeaderValue::from_str(&format!("OAuth {}", AUTHORIZATION_TOKEN))?;

    let response_text = client
        .get("https://cloud-api.yandex.net/v1/disk/")
        .header(AUTHORIZATION, auth_header)
        .send()?
        .error_for_status()?
        .text()?;

    let json: Value = serde_json::from_str(&response_text)?;
    let pretty = serde_json::to_string_pretty(&json)?;

    println!("{pretty}");

    Ok(())
}