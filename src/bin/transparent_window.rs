//! Semi transparent window, without decoration
//!
//!
#![crate_type = "bin"]

extern crate gtk;

use gtk::prelude::*;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = gtk::Window::new(gtk::WindowType::Toplevel);

    window.set_title("Semi transparent Window");
    window.set_border_width(100);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(800, 600);
    window.set_decorated(false);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    let button = gtk::Button::new_with_label("close");

    button.connect_clicked(|_| {
        gtk::main_quit();
    });

    window.add(&button);

    window.show_all();

    // only works when called after .show_all()
    window.set_opacity(0.5);

    gtk::main();
}
