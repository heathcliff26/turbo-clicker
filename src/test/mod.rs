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
    let tmp_dir = "/tmp/turbo-clicker-tests";

    unsafe {
        env::set_var(state::XDG_STATE_HOME, tmp_dir);
    }

    i_slint_backend_testing::init_no_event_loop();
    let app = AppWindow::new().expect("Should create AppWindow");
    let autoclicker_delay = Arc::new(AtomicU64::new(1000));

    register_settings_changed(&app, autoclicker_delay.clone());

    assert!(
        !fs::exists(tmp_dir).expect("Should check if directory exists"),
        "State directory should not exist initially"
    );

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

    fs::remove_dir_all(tmp_dir).expect("Should remove temporary directory");
    unsafe {
        env::remove_var(state::XDG_STATE_HOME);
    }
}
