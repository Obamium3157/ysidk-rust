use std::{fs::File, io::Read};

use reqwest::{blocking::Body, header::AUTHORIZATION};
use serde::Deserialize;
use serde_json::Value;

use crate::session::Session;

pub type Res<T> = anyhow::Result<T>;

const GET_PATH: &'static str = "https://cloud-api.yandex.net/v1/disk/";
const PUT_PATH: &'static str = "https://cloud-api.yandex.net/v1/disk/resources";

#[derive(Deserialize)]
#[allow(dead_code)]
struct UploadLink {
    href: String,
    method: String,
}

pub fn get_disk_content(session: &Session) -> Res<String> {
    let contents = session
        .client()
        .get(GET_PATH)
        .header(AUTHORIZATION, session.auth_header())
        .send()?
        .error_for_status()?
        .text()?;

    Ok(contents)
}

pub fn create_folder_on_disk(session: &Session, folder: &str) -> Res<()> {
    let path = disk_path_from_path(folder);
    let response = session
        .client()
        .put(PUT_PATH)
        .header(AUTHORIZATION, session.auth_header())
        .query(&[("path", path)])
        .send()?;

    if response.status().is_success() || response.status().as_u16() == 409 {
        println!("Папка {folder} успешно создана!");
        Ok(())
    } else {
        Err(anyhow::anyhow!(
            "Не удалось создать папку: HTTP {}",
            response.status()
        ))
    }
}

pub fn upload_file_to_disk(session: &Session, path: &str, destination: &str) -> Res<()> {
    let (client, auth_header) = (session.client(), session.auth_header());

    let parts = destination.split("/");
    let mut final_path = String::new();

    for part in parts.into_iter() {
        final_path += part;

        if !is_folder(part) {
            break;
        }

        final_path += "/";
        create_folder_on_disk(&session, &final_path)?;
    }

    let upload_url: UploadLink = client
        .get(PUT_PATH.to_owned() + "/upload")
        .header(AUTHORIZATION, auth_header)
        .query(&[
            ("path", disk_path_from_path(&final_path)),
            ("overwrite", "true".to_owned()),
        ])
        .send()?
        .error_for_status()?
        .json()?;

    let mut file = File::open(file_path_from_path(path))?;
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

fn is_folder(path: &str) -> bool {
    path.split(".").collect::<Vec<&str>>().len() < 2
}

fn file_path_from_path(path: &str) -> String {
    format!("./resources/{}", path)
}

fn disk_path_from_path(path: &str) -> String {
    format!("disk:/{}", path)
}

pub fn json_from_text(text: &str) -> Result<Value, serde_json::Error> {
    serde_json::from_str(text)
}
