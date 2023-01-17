#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod popup;
use std::{fs, process::Command, thread, path::Path};
use tauri::command;

#[macro_use]
extern crate handy_macros;

#[command]
fn report_backend(data: String) -> String {
    match data.as_str() {
        "comp/website" => return s!("comp/website"),
        "comp/not_imp" => return s!("comp/not_imp"),
        "comp/fetcherr" => return s!("comp/fetcherr"),
        "comp/settings" => return s!("comp/settings"),


        _ => {
            s!("comp/err")
        }
    }
}
const MODDIR: &'static str = "./mods";
#[command]
fn fetch_modlist() -> Vec<String> {
    let paths = fs::read_dir(MODDIR).unwrap();
    let mut mods = Vec::new();
    for path in paths {
        let modf = path.unwrap().path();
        mods.push(modf.to_str().unwrap().to_string());
    }
    mods
}
#[command]
fn correct_pos() -> bool {
    Path::new(MODDIR).exists()
}
#[command]
fn start(data: Vec<String>) {
    let mut s = String::new();
    for x in data {
        s.push_str(&format!("--mod {}  ", x));
    }

    println!("{}", s);
    let _ = thread::spawn(|| {
        let _ = Command::new(format!("{}", "./SymphonyOfEmpires"))
            .arg(s)
            .output()
            .unwrap();
    });
}

mod launcher_settings;
use launcher_settings::LauncherSettings;


#[command]
fn save_launcher_settings(settings: LauncherSettings) {
    let json = serde_json::to_string(&settings);
    if let Ok(json) = json {
        fs::write("./launcher_settings.json", json).unwrap();
    }
}

#[command]
fn get_launcher_settings() -> LauncherSettings {
    if Path::new("./launcher_settings.json").exists() {
        let data = fs::read_to_string("./launcher_settings.json");
        if let Ok(data) = data {
            if let Ok(settings) = serde_json::from_str(&data.as_str()) {
                return settings;
            }
        }
    }

    LauncherSettings::new()
}

use tauri::utils::config;
#[command]
fn get_data_dir() -> String {
    let (config, _) = config::parse("tauri.conf.json");
    
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            popup::open_web_git,
            popup::open_web,
            correct_pos,
            report_backend,
            fetch_modlist,
            start,
            save_launcher_settings,
            get_launcher_settings,
        ])
        .run(tauri::generate_context!("tauri.conf.json"))
        .expect("error while running tauri application");
}
