use super::*;
use serial_test::serial;
use std::{fs, time::Duration};
use tokio::time::sleep;

#[test]
#[serial]
fn get_autostart_file_uses_config_home() {
    let expected_dir = "some/path";
    unsafe {
        env::set_var(XDG_CONFIG_HOME, expected_dir);
    }
    let expected_path = format!("{expected_dir}/autostart/{AUTOSTART_FILE_NAME}");

    let path = get_autostart_file();
    assert_eq!(expected_path, path, "Path should match");

    unsafe {
        env::remove_var(XDG_CONFIG_HOME);
    }
}

#[test]
#[serial]
fn get_autostart_file_uses_home_when_config_home_is_empty() {
    unsafe {
        env::set_var(XDG_CONFIG_HOME, "");
    }

    let home_dir = env::home_dir().expect("HOME variable should be set");

    let expected_path = format!(
        "{}/{XDG_CONFIG_HOME_DEFAULT}/autostart/{AUTOSTART_FILE_NAME}",
        home_dir.display()
    );

    let path = get_autostart_file();
    assert_eq!(expected_path, path, "Path should match");

    unsafe {
        env::remove_var(XDG_CONFIG_HOME);
    }
}

#[tokio::test]
#[serial]
async fn enable_and_disable_autostart() {
    let tmp_dir = tempfile::tempdir().expect("Should create temporary directory");
    unsafe {
        env::set_var(XDG_CONFIG_HOME, tmp_dir.path().as_os_str());
    }
    let autostart_dir = tmp_dir.path().join("autostart");
    fs::create_dir(&autostart_dir).expect("Should create autostart directory");

    let autostart_path = autostart_dir.join(AUTOSTART_FILE_NAME);
    assert!(
        !is_autostart_enabled().await,
        "Autostart should be disabled before creating the file"
    );

    enable_autostart().await.expect("Should enable autostart");
    sleep(Duration::from_millis(10)).await;

    assert!(
        is_autostart_enabled().await,
        "Autostart should be enabled after creating the file"
    );
    assert_eq!(
        AUTOSTART_FILE,
        fs::read_to_string(&autostart_path).expect("Should read autostart file")
    );

    disable_autostart().await.expect("Should disable autostart");
    sleep(Duration::from_millis(10)).await;

    assert!(
        !is_autostart_enabled().await,
        "Autostart should be disabled after removing the file"
    );
    assert!(!autostart_path.exists(), "Autostart file should be removed");

    unsafe {
        env::remove_var(XDG_CONFIG_HOME);
    }
}
