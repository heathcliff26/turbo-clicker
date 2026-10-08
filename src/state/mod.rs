use super::slint_generatedAppWindow::{AppWindow, GlobalState};
use serde::{Deserialize, Serialize};
use slint::ComponentHandle;
use std::env;
use std::error::Error;
use std::fs;
use std::path::Path;

pub const XDG_STATE_HOME_DIR: &str = "io.github.heathcliff26.turbo-clicker";
pub const XDG_STATE_HOME: &str = "XDG_STATE_HOME";
pub const XDG_STATE_HOME_DEFAULT: &str = ".local/state";
pub const HOME: &str = "HOME";
// This list needs to be kept in sync with the frontend
const LANGUAGES: &[&str] = &["en", "de"];

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
    pub dark_mode: bool,
    #[serde(default)]
    pub language: String,
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
            dark_mode: global_state.get_dark_mode(),
            language: get_language_from_index(global_state.get_selected_language()),
        }
    }

    /// Load the state from the user specific state file.
    pub fn from_file() -> Result<Option<Self>, Box<dyn Error>> {
        Self::from_path(get_state_file_path())
    }

    /// Load the state from the given file path.
    pub fn from_path<P>(path: P) -> Result<Option<Self>, Box<dyn Error>>
    where
        P: AsRef<Path>,
    {
        if !fs::exists(&path)? {
            return Ok(None);
        };
        let file = fs::File::open(&path)?;
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
        global_state.set_dark_mode(self.dark_mode);

        let lang = get_language(&self.language);
        global_state.set_selected_language(get_language_index(&lang));
    }

    /// Save the state to user specific state file.
    pub fn save_to_file(&self) -> Result<(), Box<dyn Error>> {
        let path = get_state_file_path();
        create_parent_folder_if_not_exists(&path)?;
        let file = fs::File::create(path)?;
        serde_json::to_writer(file, self)?;
        Ok(())
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            delay: 20,
            start_delay: 1,
            duration: 1,
            use_start_delay: true,
            use_duration: true,
            dark_mode: true,
            language: "".to_string(),
        }
    }
}

/// Read the XDG state directory from the environment and return the full path to the state file.
fn get_state_file_path() -> String {
    let mut path = match env::var(XDG_STATE_HOME) {
        Ok(path) if !path.is_empty() => Some(path),
        _ => None,
    };

    if path.is_none() {
        path = match env::var(HOME) {
            Ok(home) if !home.is_empty() => Some(format!("{home}/{XDG_STATE_HOME_DEFAULT}")),
            _ => None,
        }
    }

    let path = path.unwrap_or(format!("./{XDG_STATE_HOME_DEFAULT}"));

    format!("{path}/{XDG_STATE_HOME_DIR}/state.json")
}

/// Create the parent directory of the given file if it does not exist
fn create_parent_folder_if_not_exists(path: &str) -> Result<(), Box<dyn Error>> {
    let dir = std::path::Path::new(path)
        .parent()
        .ok_or("Failed to get parent directory")?
        .to_str()
        .ok_or("Failed to convert parent directory to string")?;
    if !fs::exists(dir)? {
        match fs::create_dir_all(dir) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Failed to create directory '{dir}' for states");
                return Err(Box::new(e));
            }
        };
    }
    Ok(())
}

pub fn get_language_from_app(app: &AppWindow) -> String {
    let global_state = app.global::<GlobalState>();

    let i = global_state.get_selected_language();
    get_language_from_index(i)
}

fn get_language(lang: &str) -> String {
    if !lang.is_empty() {
        return lang.to_string();
    }

    let lang = sys_locale::get_locale().unwrap_or_default();
    lang.split(&['-', '_'][..])
        .next()
        .unwrap_or_default()
        .to_string()
}

fn get_language_from_index(i: i32) -> String {
    if i < 0 || i >= LANGUAGES.len() as i32 {
        return LANGUAGES[0].to_string();
    }
    LANGUAGES[i as usize].to_string()
}

fn get_language_index(lang: &str) -> i32 {
    LANGUAGES
        .iter()
        .position(|&x| x == lang)
        .unwrap_or_default() as i32
}
