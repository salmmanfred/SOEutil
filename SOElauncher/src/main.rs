#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod game;
mod popup;
use tauri::command;

#[macro_use]
extern crate handy_macros;


#[macro_use]
extern crate lazy_static;

#[command]
fn test() {
    println!("I was invoked from JS!");
}

#[command]
fn report_backend(data: String) -> String {
    match data.as_str() {
        "comp/game" => match game::start() {
            true => return s!("comp/game"),
            false => return s!("comp/game_fail"),
        },
        "comp/open_git" => return s!("comp/open_git"),
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
