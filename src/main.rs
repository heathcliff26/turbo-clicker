// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;

mod autoclicker;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let mut autoclicker = autoclicker::Autoclicker::new()?;
    let ui = AppWindow::new()?;

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
