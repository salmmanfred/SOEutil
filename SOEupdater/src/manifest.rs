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
        let assets = json[0]["assets"].clone();
        let mut exe_path = "";
        let mut mod_path = "";


        for (a,x) in assets.as_array().unwrap().iter().enumerate(){
            match assets[a]["name"].clone().as_str().unwrap(){
                "mods.zip" =>{mod_path = assets[a]["browser_download_url"].as_str().unwrap()},
                "game.zip" => {exe_path = assets[a]["browser_download_url"].as_str().unwrap()},
                a=>{
                    println!("{a}")
                }
            }
        }

        println!("Latest version: {}", version);
      

        Self {
            version: version.to_string(),
            exe: exe_path.to_string(),
            mods: mod_path.to_string(),

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
        unzip("./game","./exe.scrap");
        unzip("./game/exe","./mods.scrap");
    }
}
fn download_e(pth: String, name: &str) {
    println!("{}",pth);
    let resp = attohttpc::get(pth).send().expect("msg");
    openfile::write_file_bytes(name, resp.bytes().unwrap()).unwrap();
}

fn unzip(overpath: &str,path: &str) -> i32 {
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
            fs::create_dir_all(&format!("{overpath}/{}",outpath.display()));
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.display(),
                file.size()
            );
            /*if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }*/
            let mut outfile = fs::File::create(&format!("{overpath}/{}",outpath.display()));
            io::copy(&mut file, &mut outfile.unwrap()).unwrap();
        }


        
    }
    return 0;
}
