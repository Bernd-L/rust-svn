fn button1_Click(object sender, EventArgs e) {
  // The found values
  let foundValues = [];


  // Get the birthday from the text field
  String bd = tfGenerieren.Text;

  // Generation started
  for (int i = 100; i < 1000; ++i) { 
    // Create a new string (which will get modified)
    String temp = "";

    // This string will get returned
    String forReturn = "";


    // For values below 100 padding is needed
    if (i < 100) {
      // For values below 10 additional padding is needed
      if (i < 10) {
        temp += "00";
      } else {
        temp += "0";
      }
    }

    // Append the integer to the string (which now might have padding)
    temp += i; 

    // Clone the string
    forReturn = (String) temp.Clone();

    // TODO find out why an s is appended
    temp += 's'; 

    // Read in the various pieces of the birthday from the text field
    temp += bd.ElementAt(0);
    temp += bd.ElementAt(1);
    temp += bd.ElementAt(3);
    temp += bd.ElementAt(4);
    temp += bd.ElementAt(8);
    temp += bd.ElementAt(9);

    // Read in the various pieces of the birthday from the text field and append them to the result
    forReturn += bd.ElementAt(0);
    forReturn += bd.ElementAt(1);
    forReturn += bd.ElementAt(3);
    forReturn += bd.ElementAt(4);
    forReturn += bd.ElementAt(8);
    forReturn += bd.ElementAt(9);

    // Append the checksum to the result
    forReturn += Checksum(temp);
    //temp.Replace('s', ((char)Checksum(temp)));                 Alte replace zeile

    // Append the value
    foundValues.append(forReturn);
  }

  // Return the results
  return foundValues;
}

private int Checksum(String svn) {
  if (svn.Length != 10) {
  return -1;
  }

  int checkSum =
    svn.ElementAt(0) * 3 +
    svn.ElementAt(1) * 7 +
    svn.ElementAt(2) * 9 +
    svn.ElementAt(4) * 5 +
    svn.ElementAt(5) * 8 +
    svn.ElementAt(6) * 4 +
    svn.ElementAt(7) * 2 +
    svn.ElementAt(8) * 1 +
    svn.ElementAt(9) * 6;

  checkSum = checkSum % 11;

  if (checkSum == 10) {
    checkSum = 0;
  }

  return checkSum;
}

private void checkTextChanged(object sender, EventArgs e) {
  if (tfPrüfen.Text.Length != 10) {
    lbValid.Text = "Needs10digits";
    return;
  }

  if ((lbValid.Text.ElementAt(4)+"").Equals(Checksum(lbValid.Text)+"")) {
    lbValid.Text = "Valid";
  } else {
    lbValid.Text = "Invalid";
  }
}
