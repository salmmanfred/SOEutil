#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]


mod popup;
use std::{fs, process::Command, thread};

use tauri::command;

#[macro_use]
extern crate handy_macros;



extern crate lazy_static;

#[command]
fn test() {
    println!("I was invoked from JS!");
    
}

#[command]
fn report_backend(data: String) -> String {
    match data.as_str() {
       
        "comp/open_git" => return s!("comp/open_git"),
        "comp/not_imp" => return s!("comp/not_imp"),
        a => {
            panic!("cannot find {a}")
        }
    }
}

#[command]
fn fetch_modlist()->Vec<String>{
    println!("l");
    let paths = fs::read_dir("./mods").unwrap();
    let mut mods = Vec::new();
    for path in paths {
        let modf = path.unwrap().path();
        mods.push(modf.to_str().unwrap().to_string());
        
    }
    mods
}
#[command]
fn start (data: Vec<String>){
    let mut s = String::new();
    for x in data{
        s.push_str(&format!("--mod {}  ",x));
    }

    println!("{}",s);
    let _ = thread::spawn(|| {
        let _ = Command::new(format!("{}", "./SymphonyOfEmpires"))
        .arg(s)
        .output()
        .unwrap();
    });
    
   
}



fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            test,
            popup::open_web_git,
            report_backend,
            fetch_modlist,
            start,
            
        ])
        .run(tauri::generate_context!("tauri.conf.json"))
        .expect("error while running tauri application");
}
