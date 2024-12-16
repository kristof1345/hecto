#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::print_stdout,
    clippy::arithmetic_side_effects,
    clippy::as_conversions,
    clippy::integer_division
)]
mod editor;
use editor::Editor;

fn main() {
    Editor::default().run(); // this isn't flipping off the borrow checker because we are not creating a variable here
                             // if we were to store the output of this in a variable, the borrow checker would be mad at us
}
