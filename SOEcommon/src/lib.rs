use std::fs;
use std::path::Path;
use std::process::Command;
pub mod servers;

pub mod common;
use crate::common::SOErr;

#[derive(Debug)]
pub struct SOE {
    pub exe_path: SoeFile,
    updater: SoeFile,
}
impl SOE {
    pub fn has_game(&self) -> bool {
        //println!("{:#?}",&self);
        self.exe_path.unwrap()
    }
    pub fn has_file(&self, s: &str) -> bool {
        match s {
            "updater" => return self.updater.unwrap(),
            _ => {
                return false;
            }
        }
    }
    pub fn launch_game(&self) -> SOErr {
        #[cfg(target_os = "windows")]
        {
            let foo = Command::new(format!("{}", self.exe_path.form()))
                .output()
                .unwrap();
            return SOErr::Ok;
        }
        #[cfg(not(target_os = "windows"))]
        {
            println!("Operating system not supported");
            return SOErr::OSNotSupported;
        }
    }
}

#[derive(Debug)]
pub enum SoeFile {
    Path(String),
    Notfound,
}
impl SoeFile {
    pub fn reg(path: String) -> SoeFile {
        if Path::new(&format!("{}", path)).exists() {
            println!("{} exists", path);
            return SoeFile::Path(path);
        } else {
            println!("{} does not exist", path);

            return SoeFile::Notfound;
        }
    }
    pub fn unwrap(&self) -> bool {
        match self {
            SoeFile::Path(_) => true,
            Notfound => false,
        }
    }
    pub fn form(&self) -> String {
        match self {
            SoeFile::Path(a) => a.to_string(),
            Notfound => {
                panic!("path does not exist");
            }
        }
    }
}

pub fn verify() -> SOE {
    let mut how_many_found = 0;
    let mut exepath = "".to_string();
    let mut game_folder = find_game(".", "soe.exe", 5, 0).0;
    println!("Game folder: {}", game_folder);
    println!(
        "Game folder exists: {}",
        Path::new(&format!("{}", game_folder)).exists()
    );
    println!(
        "Exe exists: {}",
        Path::new(&format!("{}/soe.exe", game_folder)).exists()
    );

    SOE {
        exe_path: SoeFile::reg(format!("{}/soe.exe", game_folder)),
        updater: SoeFile::reg(format!("./SOEupdater.exe")),
    }
}
fn find_game(pathl: &str, fin: &str, max: u64, cur: u64) -> (String, String) {
    if cur >= max {
        return ("".to_string(), "".to_string());
    }

    for entry in fs::read_dir(pathl).expect("folder not found") {
        let path = entry.unwrap().path();

        if path.is_dir() {
            let l = Box::new(|| {
                let mut a: Vec<char> = format!("{:#?}", path).chars().collect();
                a.remove(0);
                a.remove(a.len() - 1);
                let paths: String = a.iter().collect();

                find_game(&format!("{}", paths), fin, max, cur + 1)
            });
            let (pathl, x) = l();

            if &x == &format!("{}\"", fin) {
                return (pathl, x);
            }
        }
        if path.is_file() {
            for x in format!("{:#?}", path).split("\\") {
                if x == format!("{}\"", fin) {
                    //println!("found {}",x);
                    return (pathl.to_string(), x.to_string());
                }
            }
        }
    }
    ("".to_string(), "".to_string())
}
