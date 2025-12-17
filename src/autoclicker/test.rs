use super::*;

#[tokio::test]
async fn new_autoclicker() {
    let autoclicker = Autoclicker::new().expect("Failed to create autoclicker");

    assert!(
        !autoclicker.running.load(Ordering::SeqCst),
        "running should be false"
    );
    assert!(
        autoclicker.stopped.load(Ordering::SeqCst),
        "stopped should be true"
    );

    let enigo = autoclicker.enigo.lock().await;
    assert!(
        enigo.location().is_ok(),
        "Should be able to get mouse location"
    );
}

#[tokio::test]
async fn autoclick_should_stop_when_signaled() {
    let mut autoclicker = Autoclicker::new().expect("Failed to create autoclicker");
    let delay_ms = Arc::new(AtomicU64::new(20));

    let started = autoclicker
        .autoclick(Arc::clone(&delay_ms), None, None)
        .await;
    assert!(started, "Autoclicker should start");
    assert!(
        autoclicker.running.load(Ordering::SeqCst),
        "running should be true"
    );
    assert!(
        !autoclicker.stopped.load(Ordering::SeqCst),
        "stopped should be false"
    );

    // Wait for the loop to start, then stop it and wait out a full delay.
    // Delay should not be less than 10ms, as otherwise the timing here might not work out.
    sleep(Duration::from_millis(10)).await;
    autoclicker.running.store(false, Ordering::Release);
    sleep(Duration::from_millis(delay_ms.load(Ordering::Acquire) * 2)).await;

    assert!(
        autoclicker.stopped.load(Ordering::SeqCst),
        "stopped should be true after stopping"
    );
}

#[tokio::test]
async fn autoclick_should_not_start_if_already_running() {
    let mut autoclicker = Autoclicker::new().expect("Failed to create autoclicker");

    autoclicker.running.store(true, Ordering::SeqCst);
    let started = autoclicker
        .autoclick(Arc::new(AtomicU64::new(20)), None, None)
        .await;
    assert!(!started, "Autoclicker should not start if already running");
}

#[tokio::test]
async fn autoclick_should_not_start_when_still_running() {
    let mut autoclicker = Autoclicker::new().expect("Failed to create autoclicker");

    autoclicker.stopped.store(false, Ordering::SeqCst);
    let started = autoclicker
        .autoclick(Arc::new(AtomicU64::new(20)), None, None)
        .await;
    assert!(!started, "Autoclicker should not start if already running");
}
