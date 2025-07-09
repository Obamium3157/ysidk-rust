mod funcitons;
mod session;

use funcitons::{get_disk_content, json_from_text, upload_file_to_disk};

use crate::{funcitons::{move_file_on_disk, read_from_disk}, session::Session};

fn main() -> anyhow::Result<()> {
    let session = Session::new()?;

    // let val = json_from_text(&get_disk_content(&session)?)?;
    // println!("{:#?}", val);

    // upload_file_to_disk(&session, "test.txt", "test30/test31/test32/hehehe.txt")?;
    // upload_file_to_disk(&session, "example.zip", "test40/archive.zip")?;

    move_file_on_disk(&session, "archive.zip", "test00/test01/archive.zip")?;

    // if let Some(text) = read_from_disk(&session, "test30/test31/test32/hehehe.txt")? {
    //     println!("Содержимое файла:\n{text}");
    // } else {
    //     println!("Файл не найден");
    // }

    Ok(())
}
