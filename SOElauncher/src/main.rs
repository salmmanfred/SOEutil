#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod game;
mod popup;
use tauri::command;

#[macro_use]
extern crate lazy_static;

#[command]
fn test() {
    println!("I was invoked from JS!");
}

#[command]
fn report_backend(data: String) -> String {
    match data.as_str() {
        "comp/game" => {
            
            match game::start(){
                true => {
                    return "comp/game".to_string()
                }
                false =>{
                    return "comp/game_fail".to_string()
                }
            }
        }
        "comp/open_git" => return "comp/open_git".to_string(),
        a => {
            panic!("cannot find {a}")
            
        }
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            test,
            popup::open_web_git,
            report_backend
        ])
        .run(tauri::generate_context!("tauri.conf.json"))
        .expect("error while running tauri application");
}
