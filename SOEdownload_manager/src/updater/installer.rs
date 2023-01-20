use log::error;
use std::fs;
use std::path::PathBuf;
use zip::ZipArchive;

pub fn install_archive(archive_path: PathBuf) {
    let file = match fs::File::open(archive_path) {
        Ok(file) => file,
        Err(_) => {
            error!("Unable to open the archive, this shouldn't have happened");
            return;
        }
    };

    let mut archive = match ZipArchive::new(file) {
        Ok(archive) => archive,
        Err(_) => {
            error!("Unable to open the file as an archive");
            return;
        }
    };

    match archive.extract(".") {
        Ok(_) => {}
        Err(_) => {
            error!("Unable to extract the archive");
        }
    }
}
