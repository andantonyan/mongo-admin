extern crate gtk;

use self::gtk::prelude::*;

use self::gtk::{Button, Window, WindowType, WindowPosition};

pub fn main() {
  if gtk::init().is_err() {
    println!("Failed to initialize GTK.");
    return;
  }

  let window = Window::new(WindowType::Toplevel);
  window.set_title("Mongo admin");
  window.set_default_size(1368, 768);
  window.set_position(WindowPosition::Center);
  window.show_all();

  window.connect_delete_event(|_, _| {
    gtk::main_quit();
    Inhibit(false)
  });

  gtk::main();
}
