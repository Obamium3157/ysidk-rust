mod funcitons;
mod session;

use funcitons::{get_disk_content, json_from_text, upload_file_to_disk};

use crate::{funcitons::read_from_disk, session::Session};

fn main() -> anyhow::Result<()> {
    let session = Session::new()?;

    let val = json_from_text(&get_disk_content(&session)?)?;
    println!("{:#?}", val);

    upload_file_to_disk(&session, "test.txt", "test20/test21/hehehe.txt")?;

    if let Some(text) = read_from_disk(&session, "test20/test21/hehehe.txt")? {
        println!("Содержимое файла:\n{text}");
    } else {
        println!("Файл не найден");
    }

    Ok(())
}
