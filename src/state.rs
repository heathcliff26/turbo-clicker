use super::slint_generatedAppWindow::{AppWindow, GlobalState};
use serde::{Deserialize, Serialize};
use slint::ComponentHandle;
use std::env;
use std::error::Error;
use std::ffi::CStr;
use std::fs;
use std::mem;
use std::ptr;

#[cfg(not(test))]
static SHARED_STATE_DIR: &str = "/var/lib/io.github.heathcliff26.turbo-clicker";
#[cfg(test)]
static SHARED_STATE_DIR: &str = "testdata";

static ORIGINAL_USER_ENV_VAR: &str = "ORIGINAL_USER";

#[cfg(test)]
mod test;

/// Contains all values from GlobalState of the UI.
/// Defaults will be set in GlobalState in the UI.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct State {
    pub delay: u64,
    pub start_delay: u64,
    pub duration: u64,
    pub use_start_delay: bool,
    pub use_duration: bool,
}

impl State {
    /// Create a new State instance from the GlobalState in the App.
    pub fn from_app(app: &AppWindow) -> Self {
        let global_state = app.global::<GlobalState>();

        Self {
            delay: global_state.get_delay().try_into().unwrap(),
            start_delay: global_state.get_start_delay().try_into().unwrap(),
            duration: global_state.get_duration().try_into().unwrap(),
            use_start_delay: global_state.get_use_start_delay(),
            use_duration: global_state.get_use_duration(),
        }
    }

    /// Load the state from the user specific state file.
    pub fn from_file() -> Result<Option<Self>, Box<dyn Error>> {
        let path = get_state_file_path();
        Self::from_path(&path)
    }

    /// Load the state from the given file path.
    pub fn from_path(path: &str) -> Result<Option<Self>, Box<dyn Error>> {
        if !fs::exists(path)? {
            return Ok(None);
        };
        let file = fs::File::open(path)?;
        let state: State = serde_json::from_reader(file)?;
        Ok(Some(state))
    }

    /// Update the GlobalState in the App with this State instance.
    pub fn update_app(&self, app: &AppWindow) {
        let global_state = app.global::<GlobalState>();

        global_state.set_delay(self.delay as i32);
        global_state.set_start_delay(self.start_delay as i32);
        global_state.set_duration(self.duration as i32);
        global_state.set_use_start_delay(self.use_start_delay);
        global_state.set_use_duration(self.use_duration);
    }

    /// Save the state to user specific state file.
    pub fn save_to_file(&self) -> Result<(), Box<dyn Error>> {
        if !fs::exists(SHARED_STATE_DIR)? {
            match fs::create_dir_all(SHARED_STATE_DIR) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Failed to create folder '{SHARED_STATE_DIR}' for states");
                    return Err(Box::new(e));
                }
            };
        }

        let file = fs::File::create(get_state_file_path())?;
        serde_json::to_writer(file, self)?;
        Ok(())
    }
}

/// Combine the common directory with the invoking user's name and return the path to the state.
fn get_state_file_path() -> String {
    let mut user = match env::var(ORIGINAL_USER_ENV_VAR) {
        Ok(user) if !user.is_empty() => Some(user),
        _ => None,
    };

    if user.is_none() {
        user = get_current_user();
    }

    if let Some(user) = user {
        return format!("{SHARED_STATE_DIR}/state_{user}.json");
    }

    eprintln!("Failed to find the invoking user, falling back to unknown_user_state.json.");
    format!("{SHARED_STATE_DIR}/unknown_user_state.json")
}

/// Use libc to get the current user's name.
fn get_current_user() -> Option<String> {
    unsafe {
        let uid = libc::getuid();

        let mut passwd: libc::passwd = mem::zeroed();

        let buf_size = match libc::sysconf(libc::_SC_GETPW_R_SIZE_MAX) {
            n if n < 0 => 512,
            n => n as usize,
        };
        let mut buf = vec![0; buf_size];

        let mut result = ptr::null_mut();

        let exit_code = libc::getpwuid_r(
            uid,
            &mut passwd,
            buf.as_mut_ptr(),
            buf.capacity() as libc::size_t,
            &mut result,
        );
        if exit_code != 0 || result.is_null() {
            eprintln!(
                "Failed to get current user name: {}",
                std::io::Error::last_os_error()
            );
            return None;
        }
        match CStr::from_ptr(passwd.pw_name).to_str() {
            Ok(name) => Some(name.to_string()),
            Err(e) => {
                eprintln!("Failed to convert user name to string: {e}");
                None
            }
        }
    }
}
