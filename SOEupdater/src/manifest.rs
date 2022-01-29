use openfile;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::io;
use std::thread;

#[derive(Serialize, Deserialize, Debug)]

pub struct Man {
    pub version: String,
    exe: String,
    mods: String,
}
impl Man {
    pub fn new() -> Self {
        let strs = openfile::read_file("./mani.scrap").unwrap();

        let json: Value = serde_json::from_str(&strs).unwrap();
        let version = json[0]["tag_name"].as_str().unwrap();
        println!("Latest version: {}", version);
      

        Self {
            version: version.to_string(),
            exe: "t".to_string(),
            mods: "t".to_string(),

        }
    }
    pub fn download_exe(&self) {
        let pth = self.exe.clone();
        let l = thread::spawn(|| {
            download_e(pth, "./exe.scrap");
        });
        l.join().unwrap();
    }
    pub fn download_mods(&self) {
        let pth = self.mods.clone();
        let l = thread::spawn(|| {
            download_e(pth, "./mods.scrap");
        });
        l.join().unwrap();
    }
    pub fn unzip(&self) {
        unzip("./exe.scrap");
        unzip("./mods.scrap");
    }
}
fn download_e(pth: String, name: &str) {
    let resp = attohttpc::get(pth).send().expect("msg");
    openfile::write_file_bytes(name, resp.bytes().unwrap()).unwrap();
}

fn unzip(path: &str) -> i32 {
    let fname = std::path::Path::new(path);
    let file = fs::File::open(&fname).unwrap();

    let mut archive = zip::ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {} comment: {}", i, comment);
            }
        }

        if (&*file.name()).ends_with('/') {
            println!("File {} extracted to \"{}\"", i, outpath.display());
            fs::create_dir_all(&outpath).unwrap();
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.display(),
                file.size()
            );
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }

        // Get and Set permissions not really need since it dose not have or probably will have a linux version
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }
    return 0;
}
