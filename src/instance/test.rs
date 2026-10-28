use super::*;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn test_init_primary() {
    let tmp_dir = tempfile::tempdir().expect("Should create temporary directory");
    unsafe {
        env::set_var(XDG_RUNTIME_DIR, tmp_dir.path());
    }
    let instance = Instance::init("test").expect("Should return instance");
    assert_eq!(
        instance.path,
        format!("{}/test.sock", tmp_dir.path().to_str().unwrap()),
        "Should have expected path"
    );
    assert!(instance.listener.is_some(), "Should have listener");
    assert!(instance.is_primary(), "Should be primary");
    unsafe {
        env::remove_var(XDG_RUNTIME_DIR);
    }
}

#[tokio::test]
#[serial]
async fn test_init_secondary() {
    let tmp_dir = tempfile::tempdir().expect("Should create temporary directory");
    unsafe {
        env::set_var(XDG_RUNTIME_DIR, tmp_dir.path());
    }

    let socket_path = tmp_dir.path().join("test.sock");
    let listener =
        UnixListener::bind(&socket_path).expect("Should bind socket for secondary instance");
    drop(listener);

    let instance = Instance::init("test").expect("Should return secondary instance");
    assert_eq!(
        instance.path,
        socket_path.to_string_lossy(),
        "Should keep the expected socket path"
    );
    assert!(
        instance.listener.is_none(),
        "Should not create a listener for a secondary instance"
    );
    assert!(!instance.is_primary(), "Should not be primary");

    unsafe {
        env::remove_var(XDG_RUNTIME_DIR);
    }
}

#[tokio::test]
#[serial]
async fn test_init_error() {
    let file = tempfile::NamedTempFile::new().expect("Should create temporary file");
    unsafe {
        env::set_var(XDG_RUNTIME_DIR, file.path());
    }

    Instance::init("test")
        .expect_err("Should return a bind error for a non-directory runtime path");

    unsafe {
        env::remove_var(XDG_RUNTIME_DIR);
    }
}

#[tokio::test]
#[serial]
#[should_panic(expected = "XDG_RUNTIME_DIR is not set")]
async fn test_init_panic() {
    unsafe {
        env::remove_var(XDG_RUNTIME_DIR);
    }
    let _ = Instance::init("test");
}
