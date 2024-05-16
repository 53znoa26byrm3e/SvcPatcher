// std lib
use std::io;

// lib
use winreg::{RegKey};
use winreg::enums::{KEY_SET_VALUE, HKEY_LOCAL_MACHINE};

fn main() {

    let mut ram: String = String::new();
    println!("How many go of ram do you have : ");
    io::stdin().read_line(&mut ram).unwrap();

    let mram: u64 = match ram.trim().parse() {
        Ok(mram) => mram,
        Err(_) => {
            println!("Error while converting");
            return;
        }
    };

    let fram: &str = &*(mram * 1024 * 1024).to_string();

    let hkcu = RegKey::predef(HKEY_LOCAL_MACHINE);
    let key = hkcu.open_subkey_with_flags(r"SYSTEM\ControlSet001\Control", KEY_SET_VALUE).unwrap();
    key.set_value("SvcHostSplitThresholdInKB", &fram).expect("Error while editing regedit key");
}