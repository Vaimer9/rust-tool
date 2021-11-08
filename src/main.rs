mod stuff;
use stuff::functions::{figlet, clear_screen, input};
use std::{thread, time};
fn main() {
    loop {
        clear_screen();
        figlet("Tool");
        println!(" 1. System Info");
        println!(" 2. Quit \n");
        let theinput = input();
        let time_sleep = time::Duration::from_millis(2000);
        match theinput.as_str().trim() {
            "1" => {println!(" This Works"); break},
            _ => {println!("\n Option not available"); thread::sleep(time_sleep);}
        }
    }
}