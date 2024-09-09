use cursive::views::TextView; //Views form main bldg blocks of a Cursive TUI program
                              //cursive::views contains 'pre built views' like buttons, checkboxes, etc.

fn main() {
    let mut siv = cursive::default(); //creates a Cursive root object

    let fox_text = "Meow!
    \\
     \\
       /\\_/\\
      ( o  o  )
      =(  I  )=";

    //declare app layout - holds fox ASCII art, has to be added as layer to be visible in main program
    siv.add_layer(TextView::new(fox_text));

    siv.run(); //starts main event loop
}
