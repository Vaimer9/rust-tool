use super::functions::{clear_screen, exit, figlet, input, sleep_ms};
use sysinfo::{System, SystemExt};

pub fn sysinfo() {
    let mut sys = System::new_all();
    sys.refresh_all();
    loop {
        clear_screen();
        figlet("Sysinfo");
        println!(" System name: {:?}", sys.name().unwrap());
        println!(
            " System Kernel Version: {:?}",
            sys.kernel_version().unwrap()
        );
        println!(" System OS Version: {:?}", sys.os_version().unwrap());
        println!(" System Host Name: {:?}", sys.host_name().unwrap());
        println!(" Total RAM: {:?} GB", sys.total_memory() / (1024 * 1024));
        println!(" Number Of Processors: {:?} \n", sys.processors().len());
        println!(" 1. Go Back");
        println!(" 2. Quit \n");
        let inputsysinfo = input();

        match inputsysinfo.as_str().trim() {
            "1" => {
                return;
            }
            "2" => {
                exit();
            }
            _ => {
                println!("\n Option not available");
                sleep_ms(2000)
            }
        }
    }
}
