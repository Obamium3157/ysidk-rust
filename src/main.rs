mod funcitons;
mod session;

use funcitons::{create_folder_on_disk, get_disk_content, json_from_text, upload_file_to_disk};

use crate::session::Session;

fn main() -> anyhow::Result<()> {
    let session = Session::new()?;

    let val = json_from_text(&get_disk_content(&session)?)?;
    println!("{:#?}", val["system_folders"]);

    upload_file_to_disk(&session, "test.txt", "test11/test12/example.txt")?;


    Ok(())
}
