use super::functions::{clear_screen, exit, figlet, input, sleep_ms};
use rand::distributions::DistString;
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;

pub fn discord() {
    loop {
        clear_screen();
        figlet("Discord");
        println!(" This tool was only made for educational purposes only. \n");
        println!(" 1. Discord Nitro Gen + Checker");
        println!(" 2. Go Back");
        println!(" 3. Quit \n");
        let discordinput = input("Enter your option: ");

        match discordinput.as_str().trim() {
            "1" => {
                println!("the test");
            }
            "2" => {
                return;
            }
            "3" => {
                exit();
            }
            _ => {
                println!("\n Option not available");
                sleep_ms(2000);
            }
        }
    }
}



pub fn nitro() {
    loop {
        clear_screen();
        figlet("Nitro");
    }
}