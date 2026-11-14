use super::*;
use crate::autoclicker::Autoclicker;
use serial_test::serial;
use std::sync::atomic::Ordering;
use std::{env, fs, time::Duration};
use tokio::time::sleep;

#[test]
#[serial]
fn test_init_global_state() {
    unsafe {
        env::set_var(state::XDG_STATE_HOME, "testdata");
    }
    i_slint_backend_testing::init_no_event_loop();
    let app = AppWindow::new().expect("Should create AppWindow");
    let global_state = app.global::<GlobalState>();

    assert_eq!(20, global_state.get_delay(), "Delay should be default");
    init_global_state(&app);
    assert_eq!(
        500,
        global_state.get_delay(),
        "Delay should be updated from state file"
    );

    unsafe {
        env::remove_var(state::XDG_STATE_HOME);
    }
}

#[tokio::test]
async fn test_register_start_auto_click() {
    let autoclicker = Autoclicker::new().expect("Failed to create autoclicker");
    let autoclicker_delay = Arc::new(AtomicU64::new(1200));

    i_slint_backend_testing::init_no_event_loop();
    let app = AppWindow::new().expect("Should create AppWindow");

    register_start_auto_click(&app, autoclicker.clone(), autoclicker_delay);

    assert!(
        !autoclicker.is_running(),
        "Autoclicker should not be running initially"
    );

    app.global::<GlobalState>().invoke_start_auto_click();

    sleep(Duration::from_millis(100)).await;
    assert!(
        autoclicker.is_running(),
        "Autoclicker should be running after callback"
    );
    sleep(Duration::from_secs(2)).await; // 2100ms since start
    assert!(!autoclicker.is_running(), "Autoclicker should have stopped");
}

#[test]
#[serial]
fn test_register_settings_changed() {
    let tmp_dir = tempfile::tempdir().expect("Should create temporary directory");

    unsafe {
        env::set_var(state::XDG_STATE_HOME, tmp_dir.path());
    }

    i_slint_backend_testing::init_no_event_loop();
    let app = AppWindow::new().expect("Should create AppWindow");
    let autoclicker_delay = Arc::new(AtomicU64::new(1000));

    register_settings_changed(&app, autoclicker_delay.clone());

    app.global::<GlobalState>().invoke_settings_changed();

    assert!(
        State::from_file()
            .expect("Should load state file")
            .is_some(),
        "State should be saved to file after settings changed"
    );
    assert_eq!(
        20,
        autoclicker_delay.load(Ordering::SeqCst),
        "Autoclicker delay should be updated"
    );

    unsafe {
        env::remove_var(state::XDG_STATE_HOME);
    }
}

#[test]
#[serial]
fn test_register_change_autostart() {
    let tmp_dir = tempfile::tempdir().expect("Should create temporary directory");
    unsafe {
        env::set_var(autostart::XDG_CONFIG_HOME, tmp_dir.path());
    }
    let autostart_dir = tmp_dir.path().join("autostart");
    fs::create_dir(&autostart_dir).expect("Should create autostart directory");

    let autostart_path = autostart_dir.join(autostart::AUTOSTART_FILE_NAME);
    assert!(
        !autostart::is_autostart_enabled(),
        "Autostart should be disabled before creating the file"
    );

    i_slint_backend_testing::init_no_event_loop();
    let app = AppWindow::new().expect("Should create AppWindow");

    register_change_autostart(&app);

    app.global::<GlobalState>().set_autostart(true);
    app.global::<GlobalState>().invoke_change_autostart();

    assert!(
        autostart::is_autostart_enabled(),
        "Autostart should be enabled after creating the file"
    );
    assert_eq!(
        autostart::AUTOSTART_FILE,
        fs::read_to_string(&autostart_path).expect("Should read autostart file")
    );

    app.global::<GlobalState>().set_autostart(false);
    app.global::<GlobalState>().invoke_change_autostart();

    assert!(
        !autostart::is_autostart_enabled(),
        "Autostart should be disabled after removing the file"
    );
    assert!(!autostart_path.exists(), "Autostart file should be removed");

    unsafe {
        env::remove_var(autostart::XDG_CONFIG_HOME);
    }
}
