#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod popup;

#[macro_use]
extern crate lazy_static;


#[tauri::command]
fn test() {
  println!("I was invoked from JS!");
}



fn main() {
  tauri::Builder::default().invoke_handler(tauri::generate_handler![test,popup::popup,popup::get_cont])
    .run(tauri::generate_context!(
      "tauri.conf.json"
    ))
    .expect("error while running tauri application");
}
