// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use evdev::uinput::VirtualDevice;
use evdev::{AttributeSet, KeyCode, KeyEvent};

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    ui.on_start_auto_click(start_auto_clicker);

    ui.run()?;

    Ok(())
}

/// Starts an auto clicker in the background, simulating left mouse clicks with the given delay (ms).
/// Returns a handle to stop the clicker via the returned `watch::Sender<bool>`.
fn start_auto_clicker(delay_ms: i32, duration: i32, start_delay: i32) {
    let stop = Arc::new(Mutex::new(false));
    let stop_clone = Arc::clone(&stop);

    let mut device = VirtualDevice::builder()
        .expect("Failed to create virtual device")
        .name("turbo-clicker-mouse")
        .with_keys(&AttributeSet::from_iter([KeyCode::BTN_LEFT]))
        .expect("Failed to add BTN_LEFT key")
        .build()
        .expect("Failed to build virtual device");

    thread::spawn(move || {
        let stop = Arc::clone(&stop_clone);
        println!("Waiting for clicker to start");
        thread::sleep(Duration::from_secs(start_delay.try_into().unwrap()));
        println!("Starting loop");
        loop {
            if *stop.lock().unwrap() {
                println!("Received stop signal");
                break;
            }
            println!("Clicking...");

            device
                .emit(&[*KeyEvent::new(KeyCode::BTN_LEFT, 1)])
                .expect("Failed to emit key press");
            device
                .emit(&[*KeyEvent::new(KeyCode::BTN_LEFT, 2)])
                .expect("Failed to emit key release");

            println!("Clicked");
            thread::sleep(Duration::from_millis(delay_ms.try_into().unwrap()));
        }
    });

    thread::spawn(move || {
        println!("Waiting for duration to end");
        thread::sleep(Duration::from_secs(
            (duration + start_delay).try_into().unwrap(),
        ));
        println!("Sending stop signal");
        let mut stop = stop.lock().unwrap();
        *stop = true;
    });
}
