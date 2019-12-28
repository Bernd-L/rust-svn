extern crate gtk;
use gtk::prelude::*;

pub fn main() {
  // Initialize GTK
  if gtk::init().is_err() {
    // When something went wrong
    println!("Failed to initialize GTK.");
    return;
  }

  // Create a window based on the Glade XML file
  let glade_src = include_str!("layout/Design.glade");
  let builder = gtk::Builder::new_from_string(glade_src);

  let window: gtk::Window = builder.get_object("window1").unwrap();
  let bt_fix: gtk::Button = builder.get_object("btFix").unwrap();
  // let dialog: gtk::MessageDialog = builder.get_object("messagedialog1").unwrap();

  bt_fix.connect_clicked(move |_| println!("button clicked"));

  // button.connect_clicked(move |_| {
  //   dialog.run();
  //   dialog.hide();
  // });

  window.show_all();

  gtk::main();
}
