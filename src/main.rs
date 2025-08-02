#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]
// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use state::State;
use std::error::Error;

mod autoclicker;
mod state;

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

    let app = AppWindow::new()?;
    app.global::<GlobalState>().set_version(VERSION.into());

    let state = match State::from_file() {
        Ok(state) => state,
        Err(e) => {
            eprintln!("Failed to load state: {e}");
            None
        }
    };
    if let Some(state) = state {
        state.update_app(&app);
    }

    app.global::<GlobalState>().on_start_auto_click({
        let app = app.as_weak();
        move || {
            let app = app.unwrap();
            let global_state = app.global::<GlobalState>();

            let start_delay: Option<u64> = match global_state.get_use_start_delay() {
                true => Some(global_state.get_start_delay().try_into().unwrap()),
                false => None,
            };
            let duration: Option<u64> = match global_state.get_use_duration() {
                true => Some(global_state.get_duration().try_into().unwrap()),
                false => None,
            };

            autoclicker.autoclick(
                global_state.get_delay().try_into().unwrap(),
                start_delay,
                duration,
            );
        }
    });

    app.run()?;

    let state = State::from_app(&app);
    if let Err(e) = state.save_to_file() {
        eprintln!("Failed to save state: {e}");
    }

    Ok(())
}
