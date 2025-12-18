#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]
// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use state::State;
use std::error::Error;
use std::sync::{Arc, atomic::AtomicU64, atomic::Ordering};

mod autoclicker;
mod hotkey;
mod state;

#[cfg(test)]
mod test;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const APP_ID: &str = concat!("io.github.heathcliff26.", env!("CARGO_PKG_NAME"));

slint::include_modules!();

// Need 2 threads here, one will be blocked by the Slint event loop.
#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() -> Result<(), Box<dyn Error>> {
    let autoclicker = match autoclicker::Autoclicker::new() {
        Ok(ac) => ac,
        Err(e) => {
            eprintln!("Failed to initialize autoclicker: {e}");
            std::process::exit(1);
        }
    };

    let app = AppWindow::new()?;
    app.global::<GlobalState>().set_version(VERSION.into());

    slint::set_xdg_app_id(APP_ID).expect("Failed to set XDG app ID");

    init_global_state(&app);

    let autoclicker_delay: u64 = app.global::<GlobalState>().get_delay().try_into().unwrap();
    let autoclicker_delay = Arc::new(AtomicU64::new(autoclicker_delay));

    let global_hotkey = hotkey::HotkeyPortal::register().await?;
    autoclicker
        .trigger_on_hotkey(global_hotkey.clone(), Arc::clone(&autoclicker_delay))
        .await?;

    register_start_auto_click(&app, autoclicker, autoclicker_delay.clone());
    register_settings_changed(&app, autoclicker_delay);
    register_configure_hotkey(&app, global_hotkey);

    app.run()?;

    save_global_state(&app);

    Ok(())
}

/// Initialize the global state from the saved state file.
fn init_global_state(app: &AppWindow) {
    let state = match State::from_file() {
        Ok(state) => state,
        Err(e) => {
            eprintln!("Failed to load state: {e}");
            None
        }
    };
    if let Some(state) = state {
        state.update_app(app);
    }
}

/// Save the global state to file.
fn save_global_state(app: &AppWindow) {
    let state = State::from_app(app);
    if let Err(e) = state.save_to_file() {
        eprintln!("Failed to save state: {e}");
    }
}

/// Register the callback for clicking the "Start Auto-click" button.
fn register_start_auto_click(
    app: &AppWindow,
    autoclicker: autoclicker::Autoclicker,
    autoclicker_delay: Arc<AtomicU64>,
) {
    let app_weak = app.as_weak();

    app.global::<GlobalState>().on_start_auto_click({
        move || {
            let app = app_weak.unwrap();
            let global_state = app.global::<GlobalState>();

            let start_delay: Option<u64> = match global_state.get_use_start_delay() {
                true => Some(global_state.get_start_delay().try_into().unwrap()),
                false => None,
            };
            let duration: Option<u64> = match global_state.get_use_duration() {
                true => Some(global_state.get_duration().try_into().unwrap()),
                false => None,
            };

            let delay = Arc::clone(&autoclicker_delay);

            let mut autoclicker = autoclicker.clone();
            tokio::spawn(async move {
                autoclicker.autoclick(delay, start_delay, duration).await;
            });
        }
    });
}

/// Register the callback for setting changes.
fn register_settings_changed(app: &AppWindow, autoclicker_delay: Arc<AtomicU64>) {
    let app_weak = app.as_weak();

    app.global::<GlobalState>().on_settings_changed({
        move || {
            let app = app_weak.unwrap();
            let global_state = app.global::<GlobalState>();

            autoclicker_delay.store(
                global_state.get_delay().try_into().unwrap(),
                Ordering::Release,
            );

            save_global_state(&app);
        }
    });
}

/// Register the callback for configuring the hotkey.
fn register_configure_hotkey(app: &AppWindow, global_hotkey: hotkey::HotkeyPortal) {
    app.global::<GlobalState>().on_configure_hotkey({
        move || {
            let global_hotkey = global_hotkey.clone();
            tokio::spawn(async move {
                global_hotkey.configure_hotkey().await;
            });
        }
    });
}
