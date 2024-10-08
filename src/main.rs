use cursive::views::{Checkbox, Dialog, EditView, ListView}; //Views form main bldg blocks of a Cursive TUI program
                                                            //cursive::views contains 'pre built views' like buttons, checkboxes, etc.
use cursive::traits::Nameable;
use cursive::Cursive;

//put all form field values into
//one struct -- passes easily
struct FoxsayOptions<'a> {
    message: &'a str,
    dead: bool,
}

//sets up layout for the form as well as callback fns
fn input_step(siv: &mut Cursive) {
    siv.add_layer(
        Dialog::new()
            .title("Please fill out the form for the fox")
            .content(
                ListView::new()
                    .child("Message:", EditView::new().with_name("message"))
                    .child("Dead?", Checkbox::new().with_name("dead")),
            )
            .button("OK", |k| {
                let message = k
                    .call_on_name("message", |t: &mut EditView| t.get_content())
                    .unwrap();
                let is_dead = k
                    .call_on_name("dead", |u: &mut Checkbox| u.is_checked())
                    .unwrap();
                let options = FoxsayOptions {
                    message: &message,
                    dead: is_dead,
                };
                result_step(k, &options) //2
            }),
    );
}

//hides form, shows fox and dialog after "OK" clicked
fn result_step(siv: &mut Cursive, options: &FoxsayOptions) {
    let eye = if options.dead { "x" } else { "o" };

    let fox_text = format!(
        "{msg}
    \\
     \\
       /\\_/\\
      (  {eye}  {eye}  )
      =( I )=",
        msg = options.message,
        eye = eye
    );

    siv.pop_layer(); //3
    siv.add_layer(
        //4
        Dialog::text(fox_text)
            .title("The fox says...")
            .button("OK", |k| k.quit()),
    );
}

fn main() {
    let mut siv = cursive::default(); //creates a Cursive root object

    input_step(&mut siv); //1

    siv.run(); //starts main event loop
}
