#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

#[tauri::command]
fn test() {
  println!("I was invoked from JS!");
}



fn main() {
  tauri::Builder::default().invoke_handler(tauri::generate_handler![test])
    .run(tauri::generate_context!(
      "tauri.conf.json"
    ))
    .expect("error while running tauri application");
}
