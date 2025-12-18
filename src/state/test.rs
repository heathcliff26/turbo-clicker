use super::*;
use serial_test::serial;

#[test]
fn state_from_app() {
    let expected_state = State {
        delay: 500,
        start_delay: 5,
        duration: 10,
        use_start_delay: false,
        use_duration: true,
        dark_mode: true,
    };

    i_slint_backend_testing::init_no_event_loop();
    let app = AppWindow::new().expect("Should create AppWindow");
    let global_state = app.global::<GlobalState>();
    global_state.set_delay(expected_state.delay as i32);
    global_state.set_start_delay(expected_state.start_delay as i32);
    global_state.set_duration(expected_state.duration as i32);
    global_state.set_use_start_delay(expected_state.use_start_delay);
    global_state.set_use_duration(expected_state.use_duration);
    global_state.set_dark_mode(expected_state.dark_mode);

    assert_eq!(
        expected_state,
        State::from_app(&app),
        "State should match GlobalState in App"
    );
}

#[test]
fn state_update_app() {
    let state = State {
        delay: 200,
        start_delay: 20,
        duration: 15,
        use_start_delay: true,
        use_duration: false,
        dark_mode: false,
    };

    i_slint_backend_testing::init_no_event_loop();
    let app = AppWindow::new().expect("Should create AppWindow");
    state.update_app(&app);

    let global_state = app.global::<GlobalState>();
    assert_eq!(
        state.delay as i32,
        global_state.get_delay(),
        "GlobalState delay should match State delay"
    );
    assert_eq!(
        state.start_delay as i32,
        global_state.get_start_delay(),
        "GlobalState start_delay should match State start_delay"
    );
    assert_eq!(
        state.duration as i32,
        global_state.get_duration(),
        "GlobalState duration should match State duration"
    );
    assert_eq!(
        state.use_start_delay,
        global_state.get_use_start_delay(),
        "GlobalState use_start_delay should match State use_start_delay"
    );
    assert_eq!(
        state.use_duration,
        global_state.get_use_duration(),
        "GlobalState use_duration should match State use_duration"
    );
    assert_eq!(
        state.dark_mode,
        global_state.get_dark_mode(),
        "GlobalState dark_mode should match State dark_mode"
    );
}

#[test]
#[serial]
fn state_from_not_existing_file() {
    unsafe {
        env::set_var(XDG_STATE_HOME, "/not/an/existing/directory");
    }

    let state = State::from_file().expect("Should not fail");
    assert!(state.is_none(), "Should return None for non-existing file");

    unsafe {
        env::remove_var(XDG_STATE_HOME);
    }
}

#[test]
#[serial]
fn state_from_file() {
    unsafe {
        env::set_var(XDG_STATE_HOME, "testdata");
    }

    let state = State::from_file().expect("Should not fail");
    let state = match state {
        Some(s) => s,
        None => panic!("Should return Some state"),
    };

    let expected_state = State {
        delay: 500,
        start_delay: 60,
        duration: 1,
        use_start_delay: true,
        use_duration: true,
        dark_mode: false,
    };

    assert_eq!(
        expected_state, state,
        "State loaded from file should match expected state"
    );

    unsafe {
        env::remove_var(XDG_STATE_HOME);
    }
}

#[test]
#[serial]
fn state_save_to_file() {
    let state = State {
        delay: 300,
        start_delay: 30,
        duration: 25,
        use_start_delay: true,
        use_duration: true,
        dark_mode: true,
    };

    let tmp_dir = "/tmp/turbo-clicker-tests";

    unsafe {
        env::set_var(XDG_STATE_HOME, tmp_dir);
    }

    state.save_to_file().expect("Should save state to file");

    let path = format!("{tmp_dir}/{XDG_STATE_HOME_DIR}/state.json");

    assert!(
        fs::exists(&path).expect("Should check if file exists"),
        "State file should exist after saving"
    );

    let loaded_state = match State::from_path(&path).expect("Should load state from file") {
        Some(s) => s,
        None => panic!("Should return Some state after loading"),
    };

    assert_eq!(state, loaded_state, "Loaded state should match saved state");

    fs::remove_dir_all(tmp_dir).expect("Should remove temporary directory");
    unsafe {
        env::remove_var(XDG_STATE_HOME);
    }
}

#[test]
#[serial]
fn get_state_file_path_env_set() {
    let expected_dir = "some/path";
    unsafe {
        env::set_var(XDG_STATE_HOME, expected_dir);
    }
    let expected_dir = format!("{expected_dir}/{XDG_STATE_HOME_DIR}/state.json");

    let path = get_state_file_path();
    assert_eq!(expected_dir, path, "Path should match");

    unsafe {
        env::remove_var(XDG_STATE_HOME);
    }
}

#[test]
#[serial]
fn get_state_file_path_empty_env_variable() {
    unsafe {
        env::set_var(XDG_STATE_HOME, "");
    }

    let home_dir = env::var(HOME).expect("HOME variable should be set");

    let path = get_state_file_path();
    assert_eq!(
        format!("{home_dir}/{XDG_STATE_HOME_DEFAULT}/{XDG_STATE_HOME_DIR}/state.json"),
        path,
        "Path should match"
    );

    unsafe {
        env::remove_var(XDG_STATE_HOME);
    }
}

#[test]
#[serial]
fn get_state_file_path_home_unset() {
    let home_dir = env::var(HOME).expect("HOME variable should be set");
    unsafe {
        env::set_var(XDG_STATE_HOME, "");
        env::remove_var(HOME);
    }

    let path = get_state_file_path();
    assert_eq!(
        format!("./{XDG_STATE_HOME_DEFAULT}/{XDG_STATE_HOME_DIR}/state.json"),
        path,
        "Path should match"
    );

    unsafe {
        env::set_var(HOME, home_dir);
    }
}
