extern crate gtk;

use std::io::Write;
use gtk::prelude::*;
use gtk::{Button, Window, WindowType};

fn main() {
    if gtk::init().is_err() {
        writeln!(&mut std::io::stderr(), "Failed to initialize GTK.").unwrap();

        std::process::exit(1);
    }

    let window = Window::new(WindowType::Toplevel);

    window.set_title("Image Viewer");

    window.set_default_size(500, 500);

    let button = Button::new_with_label("Close");

    window.add(&button);

    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();

        Inhibit(false)
    });

    button.connect_clicked(|_| {
        gtk::main_quit();
    });

    gtk::main();
}
