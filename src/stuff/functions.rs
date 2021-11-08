use figlet_rs::FIGfont;
use std::env;
use std::process::Command;
use std::io::{self, Write};

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

pub fn input() -> String {
    let mut input = String::new();
    print!(" Enter Your Option: ");
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut input).expect("Could Not Read Input");
    return input;
}