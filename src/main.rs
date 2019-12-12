fn main() {
  println!("Hello, world!");

  // Calculate SVN

  for (let mut i: u64 = 100; i < 1000; ++i) {

  }
}



// 

for (int i = 100; i < 1000; ++i)
            { 
                String temp = "";
                String forReturn = "";
                if (i < 100)
                {
                    if (i < 10)
                    {
                        temp += "00";
                    }
                    else
                    {
                        temp += "0";
                    }
                }
                temp += i; 
                forReturn = (String) temp.Clone();
                temp += 's'; 
                temp += bd.ElementAt(0);
                temp += bd.ElementAt(1);
                temp += bd.ElementAt(3);
                temp += bd.ElementAt(4);
                temp += bd.ElementAt(8);
                temp += bd.ElementAt(9);

                forReturn += bd.ElementAt(0);
                forReturn += bd.ElementAt(1);
                forReturn += bd.ElementAt(3);
                forReturn += bd.ElementAt(4);
                forReturn += bd.ElementAt(8);
                forReturn += bd.ElementAt(9);

                forReturn += Checksum(temp);
                //temp.Replace('s', ((char)Checksum(temp)));                 Alte replace zeile
                tfAusgabe.Text += forReturn;
                tfAusgabe.Text += "\r\n";
            }
        }
        private int Checksum(String svn)
        {
            if (svn.Length != 10)
            {
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
            if (checkSum == 10)
            {
                checkSum = 0;
            }
            return checkSum;
        }

        private void checkTextChanged(object sender, EventArgs e)
        {
            if (tfPrüfen.Text.Length != 10)
            {
                lbValid.Text = "Needs10digits";
                return;
            }
            if ((lbValid.Text.ElementAt(4)+"").Equals(Checksum(lbValid.Text)+""))
            {
                lbValid.Text = "Valid";
            }
            else
            {
                lbValid.Text = "Invalid";
            }
        }
    }
}