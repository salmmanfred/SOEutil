use SOEcommon::verify;

use tauri::command;

pub fn start() -> bool {
    println!("starting ran");
    let soe = verify();

    match soe.has_game() {
        true => {
            match soe.launch_game(){
                "ok" =>{
                    println!("started game");
                }
                "NotSupport" =>{
                    println!("Could not start game wrong OS");
                }
                _=>{
                    panic!("what");
                }
            }
            println!("found game");

            return true;
        }
        false => {
            println!("no game");

            return false;
        }
    }
}
