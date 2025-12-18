use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::time::Duration;

use enigo::{Button, Coordinate, Direction, Enigo, Mouse, Settings};
use tokio::sync::Mutex;
use tokio::time::sleep;

use futures_util::StreamExt;

use crate::hotkey::HotkeyPortal;

#[cfg(test)]
mod test;

/// Implement the autoclicker functionality
#[derive(Clone)]
pub struct Autoclicker {
    enigo: Arc<Mutex<Enigo>>,
    running: Arc<AtomicBool>,
    stopped: Arc<AtomicBool>,
}

impl Autoclicker {
    /// Create a new Autoclicker instance.
    /// This will initialize the enigo instance for virtual input.
    /// Returns an error if the enigo instance cannot be created.
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mut enigo = Enigo::new(&Settings::default())?;

        // Move the mouse slightly to ensure the permission prompt is triggered.
        enigo.move_mouse(1, 1, Coordinate::Rel)?;

        Ok(Autoclicker {
            enigo: Arc::new(Mutex::new(enigo)),
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
        delay_ms: Arc<AtomicU64>,
        start_delay: Option<u64>,
        duration: Option<u64>,
    ) -> bool {
        let running = Arc::clone(&self.running);
        let stopped = Arc::clone(&self.stopped);
        if self.is_running() || !self.is_stopped() {
            return false;
        }
        running.store(true, Ordering::SeqCst);
        stopped.store(false, Ordering::SeqCst);

        if let Some(start_delay) = start_delay {
            println!("Waiting for {start_delay} s before starting autoclicker");
            sleep(Duration::from_secs(start_delay)).await;
        }

        let enigo = Arc::clone(&self.enigo);

        tokio::spawn(async move {
            println!(
                "Autoclicker started with delay: {} ms",
                delay_ms.load(Ordering::Relaxed)
            );
            while running.load(Ordering::Relaxed) {
                if let Err(e) = enigo.lock().await.button(Button::Left, Direction::Click) {
                    eprintln!("Failed to click mouse button: {e}");
                };

                sleep(Duration::from_millis(delay_ms.load(Ordering::Acquire))).await;
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

    /// Listen to the event stream and trigger the autoclicker on each event.
    pub async fn trigger_on_hotkey(
        &self,
        portal: HotkeyPortal,
        delay_ms: Arc<AtomicU64>,
    ) -> Result<(), ashpd::Error> {
        let mut stream = portal.activated_stream().await?;
        let mut autoclicker = self.clone();
        tokio::spawn(async move {
            while stream.next().await.is_some() {
                println!("Hotkey activated");
                let started = autoclicker
                    .autoclick(Arc::clone(&delay_ms), None, None)
                    .await;
                if !started {
                    autoclicker.running.store(false, Ordering::Release);
                }
            }
        });
        Ok(())
    }

    /// Check if the autoclicker is currently running.
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }
    /// Check if the autoclicker is currently stopped.
    pub fn is_stopped(&self) -> bool {
        self.stopped.load(Ordering::SeqCst)
    }
}
