use std::io::{self, Write};
//use std::thread;
//use std::time::Duration;

use console::Term;
use dialoguer::{theme::ColorfulTheme, Select};
mod day_one;
use crate::day_one::run as run_day_one;
fn do_stuff() -> io::Result<()> {
    let term = Term::stdout();
    term.set_title("Advent of Code");
    term.write_line("Welcome to Ian's Advent of Code App")?;

    let options = vec!["Day 1"];
    let choice = Select::with_theme(&ColorfulTheme::default())
        .items(&options)
        .clear(true)
        .with_prompt("Please make a selection")
        .interact()?;

    match choice {
        0 => run_day_one(),
        _ => eprint!("oops"),
    }

    // for x in 0..10 {
    //     if x != 0 {
    //         term.move_cursor_up(1)?;
    //     }
    //     term.write_line(&format!("Counting {}/10", style(x + 1).cyan()))?;
    //     thread::sleep(Duration::from_millis(400));
    // }
    // term.show_cursor()?;
    // term.clear_last_lines(1)?;
    // term.write_line("Done counting!")?;
    // writeln!(&term, "Hello World!")?;

    // write!(&term, "To edit: ")?;
    // let res = term.read_line_initial_text("default")?;
    // writeln!(&term, "\n{}", res)?;

    Ok(())
}

fn main() {
    do_stuff().unwrap();
}
