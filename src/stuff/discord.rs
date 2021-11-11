use super::functions::{clear_screen, exit, figlet, input, sleep_ms, input_int};
use rand::distributions::DistString;
use rand::{thread_rng};
use rand::distributions::Alphanumeric;
use std::fs::{File};
use std::io::Write;
use curl::easy::Easy;

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
                nitro();
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
        let nitrocodes = input_int("How much nitro codes do you want: ");
        let mut file = File::create("nitro_codes.txt").expect("Couldn't create file");
        let mut easy = Easy::new();
        for _ in 0..nitrocodes {
           let mut rng = thread_rng();
           let mut thecode: String = Alphanumeric.sample_string(&mut rng, 19);
           let urlcode = format!("https://discord.gift/{}",thecode); 
           let checker = format!("https://discordapp.com/api/v9/entitlements/gift-codes/{}?with_application=false&with_subscription_plan=true",thecode);
           easy.url("{}",thecode).unwrap();
           println!(" {}",urlcode);
           let write_code = format!("{}\n",urlcode);
           file.write(write_code.as_bytes()).expect("Cannot write to file");
        }
        let _theinput = input("Enter your option: ");
    }
}