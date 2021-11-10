use figlet_rs::FIGfont;
use std::env;
use std::io::{self, Write};
use std::process;
use std::process::Command;
use std::{thread, time};

pub fn figlet(word: &str) {
    let font = FIGfont::standand().unwrap();
    let figure = font.convert(word);
    assert!(figure.is_some());
    println!("{}", figure.unwrap());
}

pub fn clear_screen() {
    let os = env::consts::OS;
    if os == "macos" {
        Command::new("clear").status().unwrap();
    } else if os == "windows" {
        Command::new("cls").status().unwrap();
    }
}

pub fn input(word: &str) -> String {
    let mut input = String::new();
    print!(" {}",word);
    let _ = io::stdout().flush();
    io::stdin()
        .read_line(&mut input)
        .expect("Could Not Read Input");
    return input;
}

pub fn exit() {
    println!("\n Goodbye we hope you enjoyed our program");
    let time_sleep_quit = time::Duration::from_millis(2000);
    thread::sleep(time_sleep_quit);
    process::exit(0x100);
}

pub fn sleep_ms(time: u64) {
    let time_sleep = time::Duration::from_millis(time);
    thread::sleep(time_sleep);
}
