use fltk::{app, button::Button, prelude::*, window::Window, *};
use webbrowser;
use SOEcommon::SOE;

#[derive(Debug, Clone)]
enum Message {
    None,
    Launch,
    Mess(String),
}
pub fn lui(game: SOE) {
    let app = app::App::default();
    let mut wind = Window::new(100, 100, 400, 300, "SOElauncher");
    let mut but = Button::new(160, 210, 100, 40, "Launch game");

    let mut grid = group::VGrid::new(0, 0, 400, 20, "grid");
    grid.set_params(1, 3, 0);
    let mut updatebut = Button::new(0, 0, 80, 20, "Update game");
    let mut repobut = Button::new(160, 0, 80, 20, "Github repo");

    grid.end();
    wind.end();
    wind.show();
    let (s, r) = app::channel::<Message>();
    but.emit(s.clone(), Message::Launch);
    repobut.emit(s, Message::Mess("repo".to_string()));

    if !game.has_game() {
        but.deactivate()
    } else {
        but.activate();
    }
    if !game.has_file("updater") {
        updatebut.deactivate()
    } else {
        updatebut.activate();
    }
    println!("{}", game.has_game());

    while app.wait() {
        if let Some(msg) = r.recv() {
            match msg {
                Message::Launch => {
                    game.launch_game();
                }
                Message::Mess(a) => match a.as_str() {
                    "repo" => {
                        webbrowser::open("https://github.com/symphony-of-empires/").is_ok();
                    }

                    _ => {}
                },
                _ => {
                    panic!("How did I get here?!")
                }
            }
        }
    }

    app.run().unwrap();
}
