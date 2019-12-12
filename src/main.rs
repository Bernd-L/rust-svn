use std::io::{stdin, stdout, Write};

fn main() {
  println!("Hello, world!");

  // Calculate SVN

  getBirthdayFromCli();
}

fn getBirthdayFromCli() {
  // Create a new string
  let mut birthday = String::new();

  // Prompt the user
  println!("Enter a birthday as DDMMYY");

  // Read in input
  read(&mut birthday);

  println!("{}", birthday);
}

/**
 * Read in user input from the CLI
 */
fn read(input: &mut String) {
  stdout().flush().expect("Couldn't flush");
  stdin().read_line(input).expect("Couldn't read");
}

/**
 * Calculates the value of the digit used to check
 * the validity of the number, given the number
 */
fn calculateValidationDigit(digits: &Vec<u8>) -> u8 {
  return 42;
  /*
    Example:
      1234 01 01 80

    First 3 digits:
      incrementing number starting from 100
      multiply by 3, 7, 9

    4th digit:
      validation digit

    last 6 digits:
      the birthday in DD MM YY
      multiply by 5, 8, 4, 2, 1, 6

    The formula:
      X = (1×3 + 2×7 + 3×9 + 0×5 + 1×8 + 0×4 + 1×2 + 8×1 + 0×6) % 11

      If X is 10, the incrementing digits are incremented by 1,
      and the calculation is repeated

      X = 7
  */
}
