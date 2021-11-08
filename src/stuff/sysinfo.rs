use super::functions::{figlet,clear_screen,input};
use sysinfo::{NetworkExt, NetworksExt, ProcessExt, System, SystemExt};

pub fn sysinfo() {
    let mut system = System::new_all();
    system.refresh_all();
    loop {
        clear_screen();
        figlet("Sysinfo");
        let inputsysinfo = input();
    }
}