use openfile;
use serde_json::Value;
use std::fs;
use std::io;
use std::thread;
use std::fs::File;
use std::io::Write;
use std::path::Path;

const EXEPATH: &'static str = "./exe.scrap";
const MODPATH: &'static str = "./mods.scrap";
pub const MANPATH: &'static str = "./mani.scrap";



pub struct Man {
    pub version: String,
    exe: String,
    mods: String,
}
impl Man {
    pub fn new() -> Self {
        let strs = openfile::read_file(MANPATH).unwrap();

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

            download_e(pth, EXEPATH);
        });
        l.join().unwrap();
    }
    pub fn download_mods(&self) {
        let pth = self.mods.clone();
        let l = thread::spawn(|| {
            download_e(pth, MODPATH);
        });
        l.join().unwrap();
    }
    pub fn unzip(&self) {
        unzip("./soe",EXEPATH);
        unzip("./soe/exe",MODPATH);
    }
    pub fn cleanup(&self){
        openfile::remove_file(EXEPATH);
        openfile::remove_file(MODPATH);
        openfile::remove_file(MANPATH);


    }
    
}
fn download_e(pth: String, name: &str) {
    println!("{}",pth);
    let resp = attohttpc::get(pth).send().expect("msg");
    //println!("Downloaded. Saving to file..");
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
            println!("File {} extracted to \"{}/{}\"", i,overpath ,outpath.display());
            fs::create_dir_all(&format!("{overpath}/{}",outpath.display()));
        } else {
            println!(
                "File {} extracted to \"{}/{}\" ({} bytes)",
                i,
                overpath,
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
pub fn save_file(names: &str, cont: Vec<u8>){
    
    
    let path = Path::new(&names);
    


    let mut file = File::create(&path).expect("error saving file0");


    
        file.write_all(&cont).expect("error saving file");
    
}