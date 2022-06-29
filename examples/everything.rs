extern crate quest;

use std::io;

pub fn main() -> io::Result<()> {
    // Messages.

    quest::success("Operation successful!");
    quest::error("Error: The compiler ate your laundry.");

    // Choose

    let choices = &["Well", "Brilliant", "Amazing"];
    quest::ask("How are you today?\n");
    let choice = quest::choose(Default::default(), choices)?;
    println!("It's good to see that you're {}.\n", choices[choice].to_lowercase());

    // Text

    quest::ask("What's your name? ");
    let name = quest::text()?;
    println!("Hello, {}!\n", name);

    // Password

    quest::ask("Password: ");
    let password = quest::password()?;
    println!("Correct, the password is {}.\n", password);

    // Editor

    let name = "script.py";
    let message = b"# Write a Python script.\n";
    let script = quest::editor(name, message)?;

    println!("Here's what you wrote. {{");
    for line in script.lines() {
        println!("    {}", line);
    }
    println!("}}\n");

    // Yes-No

    match loop {
        quest::ask("Are you the one? [yN] ");
        match quest::yesno(false)? {
            Some(b) => break b,
            None => (),
        }
    } {
        true => println!("No, I AM THE ONE!"),
        false => println!("I guess not."),
    }

    Ok(())
}
