mod stuff;
use stuff::functions::{figlet, clear_screen, input};
fn main() {
    clear_screen();
    figlet("Tool");
    let theinput = input();
}