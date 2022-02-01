mod manifest;
use std::env;
use SOEcommon::verify;

use std::io::{stdin,stdout,Write};
use manifest::MANPATH;



fn main() {
    let args: Vec<String> = env::args().collect();
    println!("SOE updater by Manfred");
    let soe = verify();
    let mut confirm = true;
    // checks the command line arguments 
    if args.len() >= 2{
        

        
        if args[1] == "--check_update" || args[1] == "-c" {

            
        
            let resp = attohttpc::get(
                "https://api.github.com/repos/symphony-of-empires/symphony-of-empires/releases",
            )
            .send()
            .expect("msg");
            openfile::write_file_bytes(MANPATH, resp.bytes().unwrap()).unwrap();
        
           
            let man = manifest::Man::new();
            println!("Latests version: {}",man.version);
            openfile::remove_file(MANPATH).unwrap();
            return ();
        }
        if args[1] == "--no_confirmation" || args[1] == "-nc" {
            confirm = false;
        }
    }
    download();
    if confirm{
        let mut s=String::new();
        print!("Press enter to continue...");
        let _=stdout().flush();
        stdin().read_line(&mut s).expect("");
    }
}
pub fn download(){
    // Downloads the latest version of the game
    

    println!("Downloading important files");

    let resp = attohttpc::get(
        "https://api.github.com/repos/symphony-of-empires/symphony-of-empires/releases",
    )
    .send()
    .expect("msg");
    openfile::write_file_bytes(MANPATH, resp.bytes().unwrap()).unwrap();

    println!("Done");
    let man = manifest::Man::new();
    println!("Downloading the game");
    man.download_exe();
    println!("Done");
    println!("Downloading the mod.zip");
    man.download_mods();
    println!("Done");
    man.unzip();
    println!("Cleaning up...");
    man.cleanup();
    println!("done");
    
}
