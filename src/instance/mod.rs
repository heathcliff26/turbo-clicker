use std::env;
use std::error::Error;
use std::io::ErrorKind;
use std::sync::Arc;
use tokio::net::{UnixListener, UnixStream};
use tokio::signal;
use tokio::task::JoinHandle;

pub const XDG_RUNTIME_DIR: &str = "XDG_RUNTIME_DIR";

#[cfg(test)]
mod test;

#[derive(Debug)]
pub struct Instance {
    path: String,
    listener: Option<Arc<UnixListener>>,
    listen_task: Option<JoinHandle<()>>,
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
            Ok(listener) => Some(Arc::new(listener)),
            Err(err) if err.kind() == ErrorKind::AddrInUse => None,
            Err(e) => {
                return Err(Box::new(e));
            }
        };

        let instance = Self {
            path,
            listener,
            listen_task: None,
        };

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

    /// Listen on the socket and run the given callback.
    /// Does nothing if the instance is not the primary.
    /// Will only listen once, further calls are no-op.
    pub fn listen(&mut self, callback: impl Fn(&str) + Send + 'static + Clone) {
        if self.listen_task.is_some() {
            return;
        }
        let listener = match self.listener.as_ref() {
            None => return,
            Some(listener) => listener.clone(),
        };
        let task = tokio::spawn(async move {
            while let Ok((stream, _)) = listener.accept().await {
                let mut buf = Vec::with_capacity(4096);
                let callback = callback.clone();
                tokio::spawn(async move {
                    loop {
                        if let Err(e) = stream.readable().await {
                            eprintln!("Failed to read from socket: {e}");
                            continue;
                        }
                        match stream.try_read_buf(&mut buf) {
                            Ok(0) => break,
                            Ok(n) => {
                                callback(&String::from_utf8_lossy(&buf[..n]));
                            }
                            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                                continue;
                            }
                            Err(e) => {
                                eprintln!("Error reading from socket: {e}");
                                break;
                            }
                        }
                    }
                });
            }
        });
        self.listen_task = Some(task);
    }

    /// Send the given string to the primary process.
    pub async fn send(&self, msg: &str) -> Result<(), Box<dyn Error>> {
        let stream = UnixStream::connect(&self.path).await?;
        stream.writable().await?;
        stream.try_write(msg.as_bytes())?;
        Ok(())
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        if let Some(task) = self.listen_task.take() {
            task.abort();
        }
        if self.is_primary() {
            std::fs::remove_file(&self.path).expect("Failed to remove socket");
        }
    }
}
