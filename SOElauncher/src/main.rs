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
fn report_backend(data: String) {
    match data.as_str() {
        "comp/game" => {
            game::start();
        }
        a => {
            println!("cannot find {a}")
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
