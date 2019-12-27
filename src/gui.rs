extern crate gtk;
use gtk::prelude::*;
use gtk::{ButtonsType, DialogFlags, MessageDialog, MessageType, Window};

pub fn main() {
  // // When something went wrong
  // if gtk::init().is_err() {
  //   println!("Failed to initialize GTK.");
  //   return;
  // }

  // // Assuming the init of GTK was successful
  // let popup = MessageDialog::new(
  //   None::<&Window>,
  //   DialogFlags::empty(),
  //   MessageType::Info,
  //   ButtonsType::Ok,
  //   "Hello World",
  // );

  // popup.run();

  //
  //
  //
  //
  //
  //
  //
  //
  //
  //

  if gtk::init().is_err() {
    println!("Failed to initialize GTK.");
    return;
  }
  let glade_src = include_str!("../layout/Design.glade");
  let builder = gtk::Builder::new_from_string(glade_src);

  let window: gtk::Window = builder.get_object("window1").unwrap();
  // let button: gtk::Button = builder.get_object("button1").unwrap();
  // let dialog: gtk::MessageDialog = builder.get_object("messagedialog1").unwrap();

  // button.connect_clicked(move |_| {
  //   dialog.run();
  //   dialog.hide();
  // });

  window.show_all();

  gtk::main();
}
