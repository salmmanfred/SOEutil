use SOEcommon::verify;

use tauri::command;

pub fn start() -> bool {
    println!("starting ran");
    let soe = verify();

    match soe.has_game() {
        true => {
            soe.launch_game();
            println!("found game");

            return true;
        }
        false => {
            println!("no game");

            return false;
        }
    }
}
