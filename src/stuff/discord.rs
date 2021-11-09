use super::functions::{clear_screen, exit, figlet, input, sleep_ms};

pub fn discord() {
    loop {
        clear_screen();
        figlet("Discord");
        println!(" This tool was only made for educational  purposes only. \n");
        println!(" 1. Discord Nitro Gen + Checker");
        let discordinput = input();
    }
}