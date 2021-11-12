use super::functions::{clear_screen, exit, figlet, input, input_int, sleep_ms};
use rand::distributions::Alphanumeric;
use rand::distributions::DistString;
use rand::thread_rng;
use reqwest;
use std::fs::File;
use std::io::Write;

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

#[tokio::main]
pub async fn nitro() {
    loop {
        clear_screen();
        figlet("Nitro");
        println!(" The nitro has a six second cooldown but \n sometimes you might see some rate limit's \n this program is only for education purposes \n only anything you cause with this i am not responsible for it.");
        println!(" ");
        let nitrocodes = input_int("How much nitro codes do you want: ");
        println!("");
        let mut file = File::create("nitro_codes.txt").expect("Couldn't create file");
        for _ in 0..nitrocodes {
            sleep_ms(6000);
            let mut rng = thread_rng();
            let mut thecode: String = Alphanumeric.sample_string(&mut rng, 19); // to generate uppercase, lowercase and numbers till 19 charecters
            let urlcode = format!("https://discord.gift/{}", thecode);
            let checker = format!("https://discordapp.com/api/v9/entitlements/gift-codes/{}?with_application=false&with_subscription_plan=true",thecode);
            let result = reqwest::get(checker).await;
            let statuscode = result.unwrap().status();
            if statuscode == 404 {
                let notvalid = format!("Not Valid | {}", urlcode);
                let notvalid_write = format!("\n {}", notvalid);
                println!(" {}", notvalid);
                file.write(notvalid_write.as_bytes())
                    .expect("Cannot write to file");
            } else if statuscode == 200 {
                let valid_code = format!(" Valid!! | {}", urlcode);
                let valid_code_write = format!("\n {}", valid_code);
                println!(" {}", valid_code);
                file.write(valid_code_write.as_bytes())
                    .expect("Cannot write to file");
            } else {
                let ratelimited = format!("Rate Limited | {}",urlcode);
                let ratelimited_write = format!("\n {}", ratelimited);
                println!(" {}", ratelimited);
                file.write(ratelimited_write.as_bytes())
                    .expect("Cannot write to file");
            }
        }
        println!("\n 1. Again");
        println!(" 2. Go Back");
        println!(" 3. Quit");
        let theinput = input("\n Enter your option: ");

        match theinput.as_str().trim() {
            "1" => {
                sleep_ms(1000);
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
