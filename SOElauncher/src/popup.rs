
use tauri::command;

#[command]
pub fn open_web_git(){
    webbrowser::open("https://github.com/symphony-of-empires/").is_ok();
}