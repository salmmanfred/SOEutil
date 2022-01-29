use std::fs;
use std::path::Path;
#[derive(Debug)]
pub struct SOE {
    exe_path: SoeFile,
}

#[derive(Debug)]
pub enum SoeFile {
    Path(String),
    Notfound,
}
impl SoeFile{
    pub fn reg(path: String)->SoeFile{
        if Path::new(&format!("{}", path)).exists(){
            return SoeFile::Path(path)
        }else{
            return SoeFile::Notfound
        }
    }
}

pub fn verify() -> SOE {
    let mut how_many_found = 0;
    let mut exepath = "".to_string();
    let mut game_folder = find_game(".", 5, 0).0;
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
    }
}
fn find_game(pathl: &str, max: u64, cur: u64) -> (String, String) {
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

                find_game(&format!("{}", paths), max, cur + 1)
            });
            let (pathl, x) = l();

            if &x == r#"soe.exe""# {
                return (pathl, x);
            }
        }
        if path.is_file() {
            for x in format!("{:#?}", path).split("\\") {
                if x == r#"soe.exe""# {
                    //println!("found {}",x);
                    return (pathl.to_string(), x.to_string());
                }
            }
        }
    }
    ("".to_string(), "".to_string())
}
