use std::env;
use tokio::fs;
use tokio::io::AsyncWriteExt;

#[cfg(test)]
mod test;

pub const XDG_CONFIG_HOME: &str = "XDG_CONFIG_HOME";
pub const XDG_CONFIG_HOME_DEFAULT: &str = ".config";
pub const AUTOSTART_FILE_NAME: &str = "io.github.heathcliff26.turbo-clicker.desktop";
pub const AUTOSTART_FILE: &str =
    include_str!("../../packages/io.github.heathcliff26.turbo-clicker.desktop");

/// Returns the (likely) path to the autostart file
fn get_autostart_file() -> String {
    let path = match env::var(XDG_CONFIG_HOME) {
        Ok(path) if !path.is_empty() => path,
        _ => match env::home_dir() {
            Some(home) => format!("{}/{}", home.display(), XDG_CONFIG_HOME_DEFAULT).to_string(),
            None => format!("./{XDG_CONFIG_HOME_DEFAULT}").to_string(),
        },
    };

    format!("{}/autostart/{AUTOSTART_FILE_NAME}", path).to_string()
}

/// Check if the autostart file exists.
/// Does not checks if it is correct.
pub async fn is_autostart_enabled() -> bool {
    let path = get_autostart_file();
    std::path::Path::new(&path).exists()
}

/// Enable autostart
pub async fn enable_autostart() -> Result<(), std::io::Error> {
    let path = get_autostart_file();
    let mut file = fs::File::create(path).await?;
    file.write_all(AUTOSTART_FILE.as_bytes()).await
}

/// Disable autostart
pub async fn disable_autostart() -> Result<(), std::io::Error> {
    let path = get_autostart_file();
    fs::remove_file(path).await
}
