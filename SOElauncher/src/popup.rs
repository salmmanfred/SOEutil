use tauri::{command, window::WindowBuilder, Window, WindowUrl};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

lazy_static!{

    static ref MANAGER: Mutex<PopupManager> = Mutex::new(PopupManager::new());

}
struct PopupManager{
    master: HashMap<String, (String,String)>
}
impl PopupManager{
    pub fn new()->Self{
        Self{
            master: HashMap::new(),
        }
    }
    pub fn register(&mut self, id: String, cont: String){
        self.master.insert(id,(cont,"null".to_string()));
    }
    pub fn get_cont(&self, id: String)->String{
        let (a,_) = self.master.get(&id).unwrap();
        a.to_owned()
    }
    pub fn get_resp(&self, id: String)->String{
        let (_,a) = self.master.get(&id).unwrap();
        a.to_owned()
        
    }
    
    pub fn set_resp(&mut self, id: String,resp: String){
        let (a,_) = self.master.get(&id).unwrap();
        self.master.insert(id,(a.to_owned(),resp));
        
    }
}


#[command]
pub async fn popup(id: String, typ: u8, window: Window) ->String {
    println!("popup");
    let child = WindowBuilder::new(&window, id, WindowUrl::App("popup.html".into()))
    .title(format!("Popup"))
    .skip_taskbar(false)
    .inner_size(400.0, 300.0).build();
    println!("popup2 {}",window.label());

    MANAGER.lock().unwrap().register(window.label().to_string(), "lel".to_string());


    use std::thread;
    let steal = window.label().to_owned().clone();
    let handler = thread::spawn(move || {
        let lb = steal.to_string();
        let null = "null".to_string();
        let mut resp = null.clone();
        while resp == null{
            resp =  MANAGER.lock().unwrap().get_resp(lb.clone());
            if resp != null {
                return resp
            }
        }

        "How did we get here?".to_string()

    });

    return  handler.join().unwrap();
/*

    let child = child.parent_window(window.hwnd().unwrap());
    println!("popup3");
  
    child.build().unwrap();
    println!("popup4");*/


    "null".to_string()

}




#[command]
pub async fn get_cont(wind: Window)->String{
    println!("re");
    MANAGER.lock().unwrap().get_cont(wind.label().to_string())
}