use tauri::command;

#[command]
pub fn open_web() {
    webbrowser::open("https://symphony-of-empires.com/").unwrap();
}
#[command]
pub fn open_web_git() {
    webbrowser::open("https://github.com/symphony-of-empires/").unwrap();
}
