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
    };

    i_slint_backend_testing::init_no_event_loop();
    let app = AppWindow::new().expect("Should create AppWindow");
    let global_state = app.global::<GlobalState>();
    global_state.set_delay(expected_state.delay as i32);
    global_state.set_start_delay(expected_state.start_delay as i32);
    global_state.set_duration(expected_state.duration as i32);
    global_state.set_use_start_delay(expected_state.use_start_delay);
    global_state.set_use_duration(expected_state.use_duration);

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
}

#[test]
#[serial]
fn state_from_not_existing_file() {
    unsafe {
        env::set_var(ORIGINAL_USER_ENV_VAR, "from_not_existing_file_test_user");
    }

    let state = State::from_file().expect("Should not fail");
    assert!(state.is_none(), "Should return None for non-existing file");

    unsafe {
        env::remove_var(ORIGINAL_USER_ENV_VAR);
    }
}

#[test]
#[serial]
fn state_from_file() {
    unsafe {
        env::set_var(ORIGINAL_USER_ENV_VAR, "from_file_test_user");
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
    };

    assert_eq!(
        expected_state, state,
        "State loaded from file should match expected state"
    );

    unsafe {
        env::remove_var(ORIGINAL_USER_ENV_VAR);
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
    };

    let user = "save_to_file_test_user";

    unsafe {
        env::set_var(ORIGINAL_USER_ENV_VAR, user);
    }

    state.save_to_file().expect("Should save state to file");

    let path = format!("{SHARED_STATE_DIR}/state_{user}.json");

    assert!(
        fs::exists(&path).expect("Should check if file exists"),
        "State file should exist after saving"
    );

    let loaded_state = match State::from_path(&path).expect("Should load state from file") {
        Some(s) => s,
        None => panic!("Should return Some state after loading"),
    };

    assert_eq!(state, loaded_state, "Loaded state should match saved state");

    fs::remove_file(&path).expect("Should remove state file");

    unsafe {
        env::remove_var(ORIGINAL_USER_ENV_VAR);
    }
}

#[test]
#[serial]
fn get_state_file_path_env() {
    let expected_user = "test_user";
    unsafe {
        env::set_var(ORIGINAL_USER_ENV_VAR, expected_user);
    }

    let path = get_state_file_path();
    let user = extract_user_from_path(&path);
    assert_eq!(
        expected_user, user,
        "User should match the environment variable"
    );

    unsafe {
        env::remove_var(ORIGINAL_USER_ENV_VAR);
    }
}

#[test]
#[serial]
fn get_state_file_path_empty_env() {
    unsafe {
        env::set_var(ORIGINAL_USER_ENV_VAR, "");
    }

    let path = get_state_file_path();
    let user = extract_user_from_path(&path);
    assert!(!user.is_empty(), "User should not be empty");

    unsafe {
        env::remove_var(ORIGINAL_USER_ENV_VAR);
    }
}

#[test]
#[serial]
fn get_state_file_path_libc() {
    unsafe {
        env::remove_var(ORIGINAL_USER_ENV_VAR);
    }

    let path = get_state_file_path();
    let user = extract_user_from_path(&path);
    assert!(!user.is_empty(), "User should not be empty");
}

fn extract_user_from_path(path: &str) -> String {
    let path = path
        .strip_prefix(SHARED_STATE_DIR)
        .expect("Should strip SHARED_STATE_DIR from path");
    let path = path
        .strip_suffix(".json")
        .expect("Should strip .json from path");
    let user = path
        .strip_prefix("/state_")
        .expect("Should strip '/state_' from path");
    user.to_string()
}

#[test]
#[serial]
fn get_current_user_from_libc() {
    unsafe {
        env::remove_var(ORIGINAL_USER_ENV_VAR);
    }

    let user = get_current_user();
    assert!(user.is_some(), "Should get user from libc");

    let env_user = env::var("USER");
    if let Ok(env_user) = env_user {
        assert_eq!(
            Some(env_user),
            user,
            "Should match the USER environment variable"
        );
    }
}
