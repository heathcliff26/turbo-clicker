// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;

mod autoclicker;

const VERSION: &str = env!("CARGO_PKG_VERSION");

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let mut autoclicker;
    match autoclicker::Autoclicker::new() {
        Ok(ac) => autoclicker = ac,
        Err(e) => {
            eprintln!("Error: {e}");
            eprintln!(
                "Please ensure this app has permissions to access /dev/uinput and that the uinput kernel module is loaded."
            );
            std::process::exit(1);
        }
    };

    let ui = AppWindow::new()?;
    ui.set_version(VERSION.into());

    ui.on_start_auto_click(
        move |delay: i32,
              start_delay: i32,
              duration: i32,
              use_start_delay: bool,
              use_duration: bool| {
            let start_delay: Option<u64> = match use_start_delay {
                true => Some(start_delay.try_into().unwrap()),
                false => None,
            };
            let duration: Option<u64> = match use_duration {
                true => Some(duration.try_into().unwrap()),
                false => None,
            };

            autoclicker.autoclick(delay.try_into().unwrap(), start_delay, duration);
        },
    );

    ui.run()?;

    Ok(())
}
