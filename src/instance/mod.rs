use std::env;
use std::error::Error;
use std::io::ErrorKind;
use tokio::net::UnixListener;
use tokio::signal;

pub const XDG_RUNTIME_DIR: &str = "XDG_RUNTIME_DIR";

#[cfg(test)]
mod test;

#[derive(Debug)]
pub struct Instance {
    path: String,
    listener: Option<UnixListener>,
}

impl Instance {
    /// Create and initialize a new instance.
    /// Attempts to create the socket.
    /// If the socket already exists, it will not be created.
    /// Returns all other errors.
    ///
    /// Panics if XDG_RUNTIME_DIR is not set.
    pub fn init(name: &str) -> Result<Self, Box<dyn Error>> {
        let runtime_dir = env::var(XDG_RUNTIME_DIR).expect("XDG_RUNTIME_DIR is not set");
        let path = format!("{runtime_dir}/{name}.sock");
        let listener = match UnixListener::bind(&path) {
            Ok(listener) => Some(listener),
            Err(err) if err.kind() == ErrorKind::AddrInUse => None,
            Err(e) => {
                return Err(Box::new(e));
            }
        };

        let instance = Self { path, listener };

        // Drop is not called when the program is killed with Ctrl+C
        if instance.is_primary() {
            tokio::spawn(async move {
                match signal::ctrl_c().await {
                    Ok(()) => slint::quit_event_loop().unwrap(),
                    Err(e) => {
                        eprintln!("{}", e)
                    }
                }
            });
        }

        Ok(instance)
    }

    /// Return if the instance is the primary
    pub fn is_primary(&self) -> bool {
        self.listener.is_some()
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        if self.is_primary() {
            std::fs::remove_file(&self.path).expect("Failed to remove socket");
        }
    }
}
