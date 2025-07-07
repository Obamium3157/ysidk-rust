use std::{fs::File, io::Read};

use reqwest::{blocking::{self, Body}, header::{HeaderValue, AUTHORIZATION}};
use serde::Deserialize;

const AUTHORIZATION_TOKEN: &'static str = "y0__xC0qIb_Axi85zgg4sDE4BOBkr9aha6pKslrohTx-3ivfnxOHw";

const FILE_PATH: &'static str = "./resources/example.txt";
const DISK_PATH: &'static str = "disk:/example.txt";

#[derive(Deserialize)]
#[allow(dead_code)]
struct UploadLink {
    href: String,
    method: String,
}

fn main() -> anyhow::Result<()> {
    let client = blocking::Client::new();
    let auth_header = HeaderValue::from_str(&format!("OAuth {}", AUTHORIZATION_TOKEN))?;

    let upload_url: UploadLink = client
        .get("https://cloud-api.yandex.net/v1/disk/resources/upload")
        .header(AUTHORIZATION, auth_header.clone())
        .query(&[
            ("path", DISK_PATH),
            ("overwrite", "true")
        ])
        .send()?
        .error_for_status()?
        .json()?;

    println!("URL для загрузки: {}", upload_url.href);

    let mut file  = File::open(FILE_PATH)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let _put_response = client
        .put(&upload_url.href)
        .header(AUTHORIZATION, auth_header)
        .body(Body::from(buffer))
        .send()?
        .error_for_status()?;

    println!("Файл успешно загружен!");

    Ok(())
}