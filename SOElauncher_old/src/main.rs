use std::env;
mod ui;
const VERSION: &str = "0.1";
//mod popup;
use SOEcommon::verify;

fn main() {
    let args: Vec<String> = env::args().collect();
    for x in args {
        match x {
            "-v" => {
                println!("{VERSION}");
                return ();
            }
            
            _=>{

            }
        }
    }
    ui::lui(verify());
}
