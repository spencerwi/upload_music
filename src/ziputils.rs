extern crate tree_magic;

use std::{fs::File, path::PathBuf};


pub fn is_zipfile(contents : &Vec<u8>) -> bool {
    let mimetype = tree_magic::from_u8(&contents);
    return mimetype == "application/zip";
}

pub async fn unpack_zipfile(zipfile_path: &PathBuf, output_dir: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(&zipfile_path)?;
    let mut archive = zip::ZipArchive::new(&file)?;
    let _ = tokio::fs::create_dir_all(&output_dir).await?;
    let _ = archive.extract(&output_dir)?;
    Ok(())
}
