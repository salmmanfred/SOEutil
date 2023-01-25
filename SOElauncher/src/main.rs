#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod popup;
use std::{fs, process::Command, thread, path::Path};
use tauri::command;


const MODDIR: &str = "./mods";
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
fn start(mods: Vec<String>) {
    let mut args = String::new();
    for mod_path in mods {
        args.push_str(&format!("--mod {}  ", mod_path));
    }

    println!("{}", args);
    let _ = thread::spawn(|| {
        let _ = Command::new(format!("{}", "./SymphonyOfEmpires"))
            .arg(args)
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
            if let Ok(settings) = serde_json::from_str(data.as_str()) {
                return settings;
            }
        }
    }

    LauncherSettings::new()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            popup::open_web_git,
            popup::open_web,
            correct_pos,
            fetch_modlist,
            start,
            save_launcher_settings,
            get_launcher_settings,
        ])
        .run(tauri::generate_context!("tauri.conf.json"))
        .expect("error while running tauri application");
}
