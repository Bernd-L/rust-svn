extern crate gtk;
use gtk::prelude::*;
use gtk::{ButtonsType, DialogFlags, MessageDialog, MessageType, Window};

pub fn main() {
  // When something went wrong
  if gtk::init().is_err() {
    println!("Failed to initialize GTK.");
    return;
  }

  // Assuming the init of GTK was successful
  let popup = MessageDialog::new(
    None::<&Window>,
    DialogFlags::empty(),
    MessageType::Info,
    ButtonsType::Ok,
    "Hello World",
  );

  popup.run();
}
