use SOEcommon::SOE;
use fltk::{app, prelude::*, window::Window,button::Button};



#[derive(Debug,Clone)]
enum Message{
    None,
    Launch,
}
pub fn lui(game: SOE){
    let app = app::App::default();
    let mut wind = Window::new(100, 100, 400, 300, "SOElauncher");
    let mut but = Button::new(160, 210, 100, 40, "Launch game");

    wind.end();
    wind.show();
    let (s, r) = app::channel::<Message>();
    but.emit(s, Message::Launch);
    if !game.has_game(){
        but.deactivate()
    }else{
        but.activate();
    }
    println!("{}",game.has_game());


    while app.wait(){
        
        if let Some(msg) = r.recv() {
            match msg {
                Message::Launch => {
                    game.launch_game();
                },
                _=>{
                    panic!("How did I get here?!")
                }
                
            }
        }
    }

    app.run().unwrap();
}