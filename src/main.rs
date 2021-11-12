mod stuff;
use stuff::discord::discord;
use stuff::functions::{clear_screen, exit, figlet, input, sleep_ms};
use stuff::sysinfo::sysinfo;
fn main() {
    loop {
        clear_screen();
        figlet("Tool");
        println!(" 1. System Info");
        println!(" 2. Discord");
        println!(" 3. Quit \n");
        let theinput = input("Enter your option: ");
        match theinput.as_str().trim() {
            "1" => {
                sysinfo();
            }
            "2" => {
                discord();
            }
            "3" => {
                exit();
            }
            _ => {
                println!("\n Option not available");
                sleep_ms(2000)
            }
        }
    }
}
