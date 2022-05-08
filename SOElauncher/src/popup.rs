use tauri::{command, window::WindowBuilder, Window, WindowUrl};

#[command]
pub async fn popup(id: String, window: Window) {
    println!("popup");
    let child = WindowBuilder::new(&window, id, WindowUrl::default())
    .title("Child")
    .inner_size(400.0, 300.0).build();
    /*println!("popup2");


    let child = child.parent_window(window.hwnd().unwrap());
    println!("popup3");
  
    child.build().unwrap();
    println!("popup4");*/


}