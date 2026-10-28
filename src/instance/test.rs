use super::*;
use serial_test::serial;
use std::sync::atomic::{AtomicBool, Ordering};

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

#[tokio::test]
#[serial]
async fn test_listen_send() {
    let tmp_dir = tempfile::tempdir().expect("Should create temporary directory");
    unsafe {
        env::set_var(XDG_RUNTIME_DIR, tmp_dir.path());
    }

    let called = Arc::new(AtomicBool::new(false));

    let mut primary = Instance::init("test").expect("Should create primary");
    let secondary = Instance::init("test").expect("Should create secondary");
    assert!(primary.is_primary(), "First instance should be primary");
    assert!(
        !secondary.is_primary(),
        "Second instance should be secondary"
    );

    let called_clone = called.clone();
    primary.listen(move |msg| {
        let called = called_clone.clone();
        assert_eq!(msg, "test message", "Should receive the expected message");
        called.store(true, Ordering::SeqCst);
    });

    secondary
        .send("test message")
        .await
        .expect("Should send message to primary");

    tokio::time::timeout(std::time::Duration::from_secs(1), async move {
        while !called.load(Ordering::SeqCst) {
            tokio::task::yield_now().await;
        }
    })
    .await
    .expect("Callback should be called");

    unsafe {
        env::remove_var(XDG_RUNTIME_DIR);
    }
}
