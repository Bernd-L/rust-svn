use crate::logic::{calculate_validation_digit, to_digits};
use std::io::{stdin, stdout, Write};

pub fn main() {
  // Calculate SVN

  let svn = get_svn_from_cli();
  let digits = to_digits(&svn);

  let res = calculate_validation_digit(&digits, false);
  println!("{}", &res);
}

/**
 * Reads in a SVN from the CLI
 */
fn get_svn_from_cli() -> String {
  // Create a new string
  let mut svn = String::new();

  // Prompt the user
  println!("Enter a SVN as IIICDDMMYY");

  // Read in input
  read(&mut svn);

  return String::from(svn.trim());
}

/**
 * Reads in a birthday from the CLI
 */
fn get_birthday_from_cli() -> String {
  // Create a new string
  let mut birthday = String::new();

  // Prompt the user
  println!("Enter a birthday as DDMMYY");

  // Read in input
  read(&mut birthday);

  return String::from(birthday.trim());
}

/**
 * Read in user input from the CLI
 */
fn read(input: &mut String) {
  stdout().flush().expect("Couldn't flush");
  stdin().read_line(input).expect("Couldn't read");
}
