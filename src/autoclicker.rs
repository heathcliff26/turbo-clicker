use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::sleep;

use evdev::uinput::VirtualDevice;
use evdev::{AttributeSet, KeyCode, KeyEvent};

#[cfg(test)]
mod test;

/// Implement the autoclicker functionality using evdev
#[derive(Clone)]
pub struct Autoclicker {
    device: Arc<Mutex<VirtualDevice>>,
    running: Arc<AtomicBool>,
    stopped: Arc<AtomicBool>,
}

impl Autoclicker {
    /// Create a new Autoclicker instance.
    /// This will create a virtual mouse device "turbo-clicker-mouse" that can emit key events.
    /// Will fail if the device cannot be created.
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let device = VirtualDevice::builder()?
            .name("turbo-clicker-mouse")
            .with_keys(&AttributeSet::from_iter([KeyCode::BTN_LEFT]))?
            .build()?;

        Ok(Autoclicker {
            device: Arc::new(Mutex::new(device)),
            running: Arc::new(AtomicBool::new(false)),
            stopped: Arc::new(AtomicBool::new(true)),
        })
    }

    /// Start the autoclicker with the given delay in milliseconds between clicks.
    /// If a start delay (in seconds) is provided, it will wait for it before starting.
    /// If a duration (in seconds) is provided, it will stop the autoclicker after that duration.
    /// Returns true if the autoclicker was started, false if it was already running.
    pub async fn autoclick(
        &mut self,
        delay_ms: u64,
        start_delay: Option<u64>,
        duration: Option<u64>,
    ) -> bool {
        let running = Arc::clone(&self.running);
        let stopped = Arc::clone(&self.stopped);
        if running.load(Ordering::SeqCst) || !stopped.load(Ordering::SeqCst) {
            return false;
        }
        running.store(true, Ordering::SeqCst);
        stopped.store(false, Ordering::SeqCst);

        if let Some(start_delay) = start_delay {
            println!("Waiting for {start_delay} s before starting autoclicker");
            sleep(Duration::from_secs(start_delay)).await;
        }

        let device = Arc::clone(&self.device);

        tokio::spawn(async move {
            println!("Autoclicker started with delay: {delay_ms} ms");
            while running.load(Ordering::Relaxed) {
                let mut device = device.lock().await;
                emit_click_event(&mut device, 1); // Mouse button down
                emit_click_event(&mut device, 0); // Mouse button up
                // Explicitly drop the lock before sleeping
                drop(device);

                sleep(Duration::from_millis(delay_ms)).await;
            }
            stopped.store(true, Ordering::Release);
            println!("Autoclicker stopped");
        });

        if let Some(duration) = duration {
            let running = Arc::clone(&self.running);
            tokio::spawn(async move {
                println!("Autoclicker will stop after {duration} s");
                sleep(Duration::from_secs(duration)).await;
                running.store(false, Ordering::Release);
            });
        }

        true
    }
}

fn emit_click_event(device: &mut VirtualDevice, value: i32) {
    match device.emit(&[*KeyEvent::new(KeyCode::BTN_LEFT, value)]) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Failed to emit click event: {e}");
        }
    };
}
