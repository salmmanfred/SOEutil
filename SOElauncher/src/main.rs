
use std::env;
mod ui;
const VERSION: &str = "0.0.1";

use SOEcommon::verify;

fn main() {
    let args:Vec<String> = env::args().collect();
    for x in args{
        if x == "-v"{
            println!("{VERSION}");
            return ();
        }
    }
    ui::lui(verify());
}
