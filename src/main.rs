use std::env;
use std::path::Path;
use std::thread;
use std::time::Duration;
use std::fs::File;
use std::io::{self, Read};

use libappindicator::{AppIndicator, AppIndicatorStatus};

fn get_mouse_info() -> io::Result<(u8, String)> {
    let battery_path = "/sys/bus/hid/drivers/razermouse/0003:1532:00A6.0001/charge_level";
    let mut charge = File::open(battery_path)?;
    let mut charge_content = String::new();
    charge.read_to_string(&mut charge_content)?;

    let scaled_percentage = match charge_content.trim().parse::<u8>() {
        Ok(value) => (value as f32 / 2.55) as u8,
        Err(_) => return Err(io::Error::new(io::ErrorKind::Other, "Parsing error 1")),
    };

    let battery_charging = "/sys/bus/hid/drivers/razermouse/0003:1532:00A6.0001/charge_status";
    let mut charging = File::open(battery_charging)?;
    let mut charging_content = String::new();
    charging.read_to_string(&mut charging_content)?;

    let state = match charging_content.trim().parse::<u8>() {
        Ok(0) => "discharging".to_string(),
        Ok(1) => "charging".to_string(),
        _ => return Err(io::Error::new(io::ErrorKind::Other, "Parsing error 2")),
    };

    Ok((scaled_percentage, state))
}

fn main() {
    gtk::init().expect("Failed to initialize GTK");

    let mut indicator = AppIndicator::new("libappindicator test application", "");
    indicator.set_status(AppIndicatorStatus::Active);
    
    let icon_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/icons");
    indicator.set_icon_theme_path(icon_path.to_str().expect("Failed to convert icon path to string"));
    
    let mut menu = gtk::Menu::new();
    indicator.set_menu(&mut menu);

    let update_interval = Duration::from_secs(1);
    loop {
        let battery = match get_mouse_info() {
            Ok((percentage, _state)) => percentage,
            Err(_) => 0,
        };

        let icon_number = (battery / 20).to_string();
        indicator.set_icon_full(&icon_number, &icon_number);
        indicator.set_title(&format!("{}% Battery", battery));

        gtk::main_iteration();
        thread::sleep(update_interval);
    }
}
