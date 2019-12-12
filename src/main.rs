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
fn calculateValidationDigit(digits: &[u8]) -> u8 {
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

  /**
   * The array after multiplication
   */
  let mut transformed_digits = [0; 10];

  // Transform the incrementing digits
  transformed_digits[0] = digits[0] * 3;
  transformed_digits[1] = digits[1] * 7;
  transformed_digits[2] = digits[2] * 9;

  // Transform the birthday digits
  // TODO cache these calculations
  transformed_digits[4] = digits[4] * 5;
  transformed_digits[5] = digits[5] * 8;
  transformed_digits[6] = digits[6] * 4;
  transformed_digits[7] = digits[7] * 2;
  transformed_digits[8] = digits[8] * 1;
  transformed_digits[9] = digits[9] * 6;

  /**
   * The sum mod 11
   */
  let x = (transformed_digits[0]
    + transformed_digits[1]
    + transformed_digits[2]
    + transformed_digits[4]
    + transformed_digits[5]
    + transformed_digits[6]
    + transformed_digits[7]
    + transformed_digits[8]
    + transformed_digits[9])
    % 11;

  // Avoid x being 10
  if x == 10 {
    // TODO increment the incrementing number (the first 3 digits) by one
    return calculateValidationDigit(&digits);
  }

  return x;
}
