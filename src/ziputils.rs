extern crate tree_magic;

use std::{fs::File, path::PathBuf};
use std::io::Cursor;
use crate::file_namer::get_filename;
use crate::audioutils;

pub fn is_zipfile(contents : &Vec<u8>) -> bool {
    let mimetype = tree_magic::from_u8(&contents);
    return mimetype == "application/zip";
}

pub async fn unpack_zipfile(zipfile_path: &PathBuf, output_dir: &PathBuf, filename_pattern: &String) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(&zipfile_path)?;
    let mut archive = zip::ZipArchive::new(&file)?;
    let _ = std::fs::create_dir_all(&output_dir);
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        if !file.is_file() {
            continue;
        }

        let mut file_contents_buffer = Vec::new();
        let _ = std::io::copy(&mut file, &mut file_contents_buffer)?;
        if !audioutils::is_supported_audiofile(&file_contents_buffer) {
            continue;
        }

        println!("Extracting {}", file.name());
        let mangled_filename = file.mangled_name();
        let extension : Option<&str> = mangled_filename.extension().and_then(|e| e.to_str());
        let mut file_contents_cursor = Cursor::new(file_contents_buffer);
        let metadata = audioutils::extract_metadata(&mut file_contents_cursor)?;
        let destination = get_filename(&metadata, filename_pattern, extension);
        if let Some(parent) = destination.parent() {
            let _ = std::fs::create_dir_all(parent)?;
        }
        let mut output_file = File::create(destination)?;
        let _ = std::io::copy(&mut file_contents_cursor, &mut output_file)?;
    }
    Ok(())
}
