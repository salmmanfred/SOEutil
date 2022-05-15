use SOEcommon::verify;

use tauri::command;
use SOEcommon::common::SOErr;


pub fn start() -> bool {
    println!("starting ran");
    let soe = verify();

    match soe.has_game() {
        true => {
            match soe.launch_game(){
                SOErr::Ok =>{
                    println!("started game");
                }
                SOErr::OSNotSupported =>{
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
