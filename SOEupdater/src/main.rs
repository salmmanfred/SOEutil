mod manifest;
// I need to stop procrastinating
use std::env;
use SOEcommon::verify;






fn main() {
    let args: Vec<String> = env::args().collect();
    println!("SOE updater by Manfred");
    let soe = verify();
    //println!("{:#?}",soe);
    // checks the command line arguments 
    if args.len() >= 2{
        

        
        if args[1] == "--check_update" || args[1] == "-c" {

            
        
            let resp = attohttpc::get(
                "https://api.github.com/repos/symphony-of-empires/symphony-of-empires/releases",
            )
            .send()
            .expect("msg");
            openfile::write_file_bytes("./mani.scrap", resp.bytes().unwrap()).unwrap();
        
           
            let man = manifest::Man::new();
            println!("Latests version: {}",man.version);
            openfile::remove_file("./mani.scrap").unwrap();
            return ();
        }
    }
    download();
}
pub fn download(){
    // Downloads the latest version of the game
    

    println!("Downloading important files");

    let resp = attohttpc::get(
        "https://api.github.com/repos/symphony-of-empires/symphony-of-empires/releases",
    )
    .send()
    .expect("msg");
    openfile::write_file_bytes("./mani.scrap", resp.bytes().unwrap()).unwrap();

    println!("Done");
    let man = manifest::Man::new();
    println!("Downloading the game");
    //man.download_exe();
    println!("Done");
    println!("Downloading the mod.zip");
    //man.download_mods();
    println!("Done");

    man.unzip();
    
}
