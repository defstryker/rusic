#![allow(dead_code)]
use std::env;
use std::rc::Rc;

use gio::prelude::*;
use gtk::prelude::*;
use gtk::Orientation::{Horizontal, Vertical};
use gtk::{
    Adjustment, Application, ApplicationWindow, ContainerExt, GtkWindowExt, Image, ImageExt, Scale,
    ScaleExt, WidgetExt, WindowPosition,
};

mod playlist;
mod toolbar;
use playlist::Playlist;
use toolbar::MusicToolbar;

struct App {
    adjustment: Adjustment,
    cover: Image,
    playlist: Rc<Playlist>,
    toolbar: MusicToolbar,
    window: ApplicationWindow,
}

impl App {
    fn new(application: &Application) -> Self {
        let window = ApplicationWindow::new(application);
        window.set_title("Rusic");
        window.set_default_size(800, 600);
        window.set_position(WindowPosition::Center);

        let vbox = gtk::Box::new(Vertical, 0);
        window.add(&vbox);

        let toolbar = MusicToolbar::new();
        vbox.add(toolbar.toolbar());

        let playlist = Rc::new(Playlist::new());
        vbox.add(playlist.view());

        let cover = Image::new();
        cover.set_from_file("cover.jpg");
        vbox.add(&cover);

        let adjustment = Adjustment::new(0.0, 0.0, 10.0, 0.0, 0.0, 0.0);
        let scale = Scale::new(Horizontal, Some(&adjustment));
        scale.set_draw_value(false);
        vbox.add(&scale);

        window.show_all();

        let app = App {
            adjustment,
            cover,
            playlist,
            toolbar,
            window,
        };

        app.connect_events();
        app.connect_toolbar_events();
        app
    }

    fn connect_events(&self) {}
}

fn main() {
    let application: Application =
        Application::new(Some("com.github.rust-by-example"), Default::default())
            .expect("Application initialization failed");

    application.connect_startup(|app| {
        let _a = App::new(app);
    });

    application.connect_activate(|_| {});

    application.run(&env::args().collect::<Vec<_>>());
}
