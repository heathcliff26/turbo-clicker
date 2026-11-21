use crate::state;
use ashpd::desktop::background::Background;
use std::error::Error;
use tokio::fs;
use tokio::io::AsyncWriteExt;

pub const AUTOSTART_STATE_FILE_NAME: &str = "autostart";

/// Returns the path to the autostart state file
fn get_autostart_state_file() -> String {
    format!(
        "{}/{AUTOSTART_STATE_FILE_NAME}",
        state::get_state_dir_path()
    )
    .to_string()
}

/// Check whether the portal will start the application automatically.
pub async fn is_autostart_enabled() -> bool {
    let path = get_autostart_state_file();
    match fs::read_to_string(&path).await {
        Ok(value) => return value == "1",
        Err(e) if e.kind() == tokio::io::ErrorKind::NotFound => return false,
        Err(e) => {
            eprintln!("Failed to read autostart state: {e}");
            return false;
        }
    }
}

/// Request that the portal starts the application automatically.
pub async fn enable_autostart() -> Result<(), Box<dyn Error>> {
    set_autostart(true).await
}

/// Request that the portal stops starting the application automatically.
pub async fn disable_autostart() -> Result<(), Box<dyn Error>> {
    set_autostart(false).await
}

async fn set_autostart(enable: bool) -> Result<(), Box<dyn Error>> {
    let response = Background::request()
        .auto_start(enable)
        .send()
        .await?
        .response()?;

    let path = get_autostart_state_file();
    let mut file = fs::File::create(path).await?;

    let state = if response.auto_start() { b"1" } else { b"0" };
    file.write_all(state).await?;
    Ok(())
}
