mod stuff;
use stuff::functions::{figlet, clear_screen, input, exit};
use stuff::sysinfo::{sysinfo};
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
            "1" => {sysinfo(); break; },
            "2" => {exit();},
            _ => {println!("\n Option not available"); thread::sleep(time_sleep);}
        }
    }
}