mod cli;
mod gui;
mod logic;

fn main() {
  // TODO Only launch GUI if CLI options are not specified
  // cli::main();
  gui::main();
}
