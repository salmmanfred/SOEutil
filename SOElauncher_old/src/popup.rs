use fltk::{app, button::Button, prelude::*, window::Window, *};
use webbrowser;
use SOEcommon::SOE;
pub fn pop(text: &str){
    let mut wind = Window::new(100, 100, 400, 300, "LUI POPUP");
    let _ = frame::Frame::default().size_of(&wind).set_label(text);
    wind.end();
    wind.show();
}