use super::functions::{figlet,clear_screen,input};
use sysinfo::{NetworkExt, NetworksExt, ProcessExt, System, SystemExt};

pub fn sysinfo() {
    let mut sys = System::new_all();
    sys.refresh_all();
    loop {
        clear_screen();
        figlet("Sysinfo");
        println!(" 1. System name:  {:?}", sys.name());
        let inputsysinfo = input();
    }
}