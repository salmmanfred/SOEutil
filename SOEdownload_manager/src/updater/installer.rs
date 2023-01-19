use std::fs;
use std::path::PathBuf;
use zip::ZipArchive;

pub fn install_archive(archive_path: PathBuf) {
    let file = fs::File::open(archive_path).unwrap();

    let mut archive = ZipArchive::new(file).unwrap();

    archive.extract(".").unwrap();
}
