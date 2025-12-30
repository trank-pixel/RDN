use chrono::{Local, Timelike};
use std::thread;
use std::time::Duration;
use winreg::enums::*;
use winreg::RegKey;

// A function to change the theme in the Windows registry 
fn update_theme() -> std::io::Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let path = r"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize";
    let (key, _) = hkcu.create_subkey(path)?;

    let hour = Local::now().hour();
    
    // Light theme from 8:00 to 20:00 (1 - light, 0 - dark)
    let is_light = if hour >= 8 && hour < 20 { 1 } else { 0 };

    key.set_value("AppsUseLightTheme", &is_light)?;
    key.set_value("SystemUsesLightTheme", &is_light)?;
    Ok(())
}

fn main() {
    println!("RDN is running and guarding your eyes! (ᵔᴥᵔ)");
    
    loop {
        if let Err(e) = update_theme() {
            eprintln!("Ошибка доступа к реестру: {}", e);
        }
        
        //Check every 5 minutes 
        thread::sleep(Duration::from_secs(300));
    }
}
