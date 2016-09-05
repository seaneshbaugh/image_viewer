extern crate cairo;
extern crate gdk;
extern crate gdk_sys;
extern crate gtk;

use std::cell::{Cell, RefCell};
use std::env;
use std::rc::Rc;
use gtk::prelude::*;
use gtk::{Button, DrawingArea, ScrolledWindow, Window, WindowType};

fn main() {
    let arguments: Vec<_> = env::args_os().collect();

    if arguments.len() != 3 {
        panic!("width and length required!");
    }

    if gtk::init().is_err() {
        panic!("Failed to initialize GTK.");
    }

    let window = Window::new(WindowType::Toplevel);

    let screen = WindowExt::get_screen(&window).unwrap();

    let screen_width = screen.get_width();

    let argument_width = arguments.get(1).unwrap().to_string_lossy().into_owned().parse::<i32>().unwrap();

    let width = {
        if argument_width > screen_width {
            screen_width
        } else {
            argument_width
        }
    };

    let screen_height = screen.get_height();

    let argument_height = arguments.get(2).unwrap().to_string_lossy().into_owned().parse::<i32>().unwrap();

    let height = {
        if argument_height > screen_height {
            screen_height
        } else {
            argument_height
        }
    };

    println!("{} x {}", width, height);

    window.set_title("Image Viewer");

    window.set_default_size(width, height);

    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 0);

    let button_box = gtk::ButtonBox::new(gtk::Orientation::Vertical);

    let zoom_in_button = Button::new_with_label("Zoom In");

    let zoom_out_button = Button::new_with_label("Zoom Out");

    let area = Rc::new(RefCell::new(DrawingArea::new()));

    let scroller = ScrolledWindow::new(None, None);

    scroller.set_size_request(300, 300);

    button_box.set_layout(gtk::ButtonBoxStyle::Start);

    button_box.pack_start(&zoom_in_button, false, false, 0);

    button_box.pack_start(&zoom_out_button, false, false, 0);

    scroller.add(&*area.borrow());

    hbox.pack_start(&scroller, false, false, 0);

    hbox.pack_start(&button_box, false, false, 0);

    window.add(&hbox);

    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();

        Inhibit(true)
    });

    let scale = Rc::new(Cell::new(1.0));

    {
        let scale = scale.clone();

        let area = area.clone();

        zoom_in_button.connect_clicked(move |_| {
            let mut s = scale.get();

            s += 0.1;

            println!("{}", s);

            scale.set(s);

            area.borrow().queue_draw();
        });
    }

    {
        let scale = scale.clone();

        let area = area.clone();

        zoom_out_button.connect_clicked(move |_| {
            let mut s = scale.get();

            s -= 0.1;

            println!("{}", s);

            scale.set(s);

            area.borrow().queue_draw();
        });
    }

    {
        let scale = scale.clone();

        let area = area.clone();

        area.borrow().connect_draw(move |this, cr| {
            let s = scale.get();

            let width : i32 = (s * (300 as f64)) as i32;

            let height : i32 = (s * (300 as f64)) as i32;

            this.set_size_request(width, height);

            cr.scale(s, s);

            cr.set_source_rgb(1.0, 1.0, 1.0);

            cr.paint();

            for x in 0..300 as usize {
                for y in 0..300 as usize {
                    let red : f64 = (((((x * y) + 100) % 255) % 256) as f64) / 255.0;
                    let blue : f64 = (((((x * y) + 200) % 255) % 256) as f64) / 255.0;
                    let green : f64 = (((((x * y) + 300) % 255) % 256) as f64) / 255.0;

                    cr.set_source_rgb(red, blue, green);

                    cr.rectangle(x as f64, y as f64, 1.0, 1.0);

                    cr.fill();
                }
            }

            Inhibit(true)
        });
    }

    area.borrow().queue_draw();

    gtk::main();
}
