#![allow(dead_code)]
use std::path::PathBuf;

use gtk::{
    ApplicationWindow, ContainerExt, DialogExt, FileChooserAction, FileChooserDialog,
    FileChooserExt, FileFilter, IconSize, Image, ImageExt, ResponseType, SeparatorToolItem,
    ToolButton, ToolButtonExt, Toolbar, WidgetExt,
};

use crate::playlist::Playlist;
use crate::App;

const PLAY_STOCK: &str = "gtk-media-play";
const PAUSE_STOCK: &str = "gtk-media-pause";

pub struct MusicToolbar {
    open_button: ToolButton,
    next_button: ToolButton,
    play_button: ToolButton,
    prev_button: ToolButton,
    quit_button: ToolButton,
    remove_button: ToolButton,
    stop_button: ToolButton,
    toolbar: Toolbar,
}

impl MusicToolbar {
    pub fn new() -> Self {
        let toolbar = Toolbar::new();

        let open_button = make_button("gtk-open");
        toolbar.add(&open_button);

        toolbar.add(&SeparatorToolItem::new());

        let prev_button = make_button("gtk-media-previous");
        toolbar.add(&prev_button);

        let play_button = make_button(PLAY_STOCK);
        toolbar.add(&play_button);

        let stop_button = make_button("gtk-media-stop");
        toolbar.add(&stop_button);

        let next_button = make_button("gtk-media-next");;
        toolbar.add(&next_button);

        toolbar.add(&SeparatorToolItem::new());

        let remove_button = make_button("gtk-remove");
        toolbar.add(&remove_button);

        toolbar.add(&SeparatorToolItem::new());

        let quit_button = make_button("gtk-quit");
        toolbar.add(&quit_button);

        MusicToolbar {
            open_button,
            next_button,
            play_button,
            prev_button,
            quit_button,
            remove_button,
            stop_button,
            toolbar,
        }
    }

    pub fn toolbar(&self) -> &Toolbar {
        &self.toolbar
    }
}

impl App {
    pub fn connect_toolbar_events(&self) {
        let window = self.window.clone();

        self.toolbar.quit_button.connect_clicked(move |_| {
            window.destroy();
        });

        let play_button = self.toolbar.play_button.clone();
        let playlist = self.playlist.clone();
        let cover = self.cover.clone();
        self.toolbar.play_button.connect_clicked(move |_| {
            let w = play_button.get_icon_widget().unwrap().get_name().unwrap();
            eprintln!("{}", w);

            if String::from(w) == PLAY_STOCK {
                let pause_image =
                    Image::new_from_icon_name(Some(PAUSE_STOCK), IconSize::LargeToolbar);
                pause_image.set_name(PAUSE_STOCK);
                play_button.set_icon_widget(Some(&pause_image));
                set_cover(&cover, &playlist);
                play_button.show_all();
            } else {
                let play_image =
                    Image::new_from_icon_name(Some(PLAY_STOCK), IconSize::LargeToolbar);
                play_image.set_name(PLAY_STOCK);
                play_button.set_icon_widget(Some(&play_image));
                play_button.show_all();
            }
        });

        let parent = self.window.clone();
        let playlist = self.playlist.clone();
        self.toolbar.open_button.connect_clicked(move |_| {
            let file = show_open_dialog(&parent);
            if let Some(file) = file {
                playlist.add(&file);
            }
        });

        let playlist = self.playlist.clone();
        self.toolbar.remove_button.connect_clicked(move |_| {
            playlist.remove_selection();
        });


        // let playlist = self.playlist.clone();
        // let play_image = self.toolbar.play_image.clone();
        // let cover = self.cover.clone();
        // let state = self.state.clone();

        // self.toolbar.play_button.connect_clicked(move |_| {
        //     if state.lock().unwrap().stopped {
        //         if playlist.play() {
        //             set
        //         }
        //     }
        // });
    }
}

/**
 * Helper Functions
 */

fn make_button(name: &str) -> ToolButton {
    let image = Image::new_from_icon_name(Some(name), IconSize::LargeToolbar);
    image.set_name(name);
    ToolButton::new(Some(&image), Some(name))
}

fn show_open_dialog(parent: &ApplicationWindow) -> Option<PathBuf> {
    let mut file = None;
    let dialog = FileChooserDialog::new(
        Some("Select an MP3 Audio file"),
        Some(parent),
        FileChooserAction::Open,
    );
    let filter = FileFilter::new();
    filter.add_mime_type("audio/mp3");
    filter.set_name(Some("MP3 audio file"));
    dialog.add_filter(&filter);
    dialog.add_button("Cancel", ResponseType::Cancel);
    dialog.add_button("Accept", ResponseType::Accept);
    let result = dialog.run();
    if result == ResponseType::Accept {
        file = dialog.get_filename();
    }
    dialog.destroy();
    file
}

fn set_cover(cover: &Image, playlist: &Playlist) {
    cover.set_from_pixbuf(playlist.pixbuf().as_ref());
    // cover.show();
}
