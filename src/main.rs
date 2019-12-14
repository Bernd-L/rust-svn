use std::io::{stdin, stdout, Write};

fn main() {
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

/**
 * Converts a string to an array of digits
 */
fn to_digits(input: &str) -> [u32; 10] {
  let chars = input.chars();
  let mut digits = [0; 10];

  let mut i = 0;
  for c in chars {
    println!("{} {}", i, c);
    digits[i] = c.to_digit(10).unwrap();
    i += 1;
  }

  return digits;
}

/**
 * Calculates the value of the digit used to check
 * the validity of the number, given the number
 */
fn calculate_validation_digit(digits: &[u32], re_run: bool) -> u32 {
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



  birthday = birthday.trim().parse().expect("Please check your input");

  */

  //  The array after multiplication
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

  // The sum mod 11
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
    return 0;
  }

  // FIXME The logic below needs to replace the if above
  /*   while x == 10 {
    // Increment the incrementing number (the first 3 digits) by one
    let mut temp: u32 = (format!("{}{}{}", digits[0], digits[1], digits[2]))
      .parse()
      .expect("Invalid format");

    temp += 1;

    digits[2] = temp % 10;
    digits[1] = temp % 100;

    // Re-run the calculation with the increased number
    return calculate_validation_digit(&digits, true);
  } */

  return x;
}

// String temp = Integer.parseInt(digits[0] + digits[1] + digits[2]) + 1 + "";
