use super::*;

#[test]
fn new_autoclicker() {
    let autoclicker = Autoclicker::new().expect("Failed to create autoclicker");

    assert!(
        !autoclicker.running.load(Ordering::SeqCst),
        "running should be false"
    );
    assert!(
        autoclicker.stopped.load(Ordering::SeqCst),
        "stopped should be true"
    );

    let mut device = autoclicker.device.lock().expect("Device lock poisoned");
    let nodes = device
        .enumerate_dev_nodes_blocking()
        .expect("Failed to enumerate device nodes");

    assert_eq!(1, nodes.count(), "There should be one device node");
}

#[test]
fn autoclick_should_stop_when_signaled() {
    let mut autoclicker = Autoclicker::new().expect("Failed to create autoclicker");
    let delay_ms = 20;

    let started = autoclicker.autoclick(delay_ms, None, None);
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
    std::thread::sleep(std::time::Duration::from_millis(10));
    autoclicker.running.store(false, Ordering::Release);
    std::thread::sleep(std::time::Duration::from_millis(delay_ms));

    assert!(
        autoclicker.stopped.load(Ordering::SeqCst),
        "stopped should be true after stopping"
    );
}

#[test]
fn autoclick_should_not_start_if_already_running() {
    let mut autoclicker = Autoclicker::new().expect("Failed to create autoclicker");

    autoclicker.running.store(true, Ordering::SeqCst);
    let started = autoclicker.autoclick(20, None, None);
    assert!(!started, "Autoclicker should not start if already running");
}

#[test]
fn autoclick_should_not_start_when_still_running() {
    let mut autoclicker = Autoclicker::new().expect("Failed to create autoclicker");

    autoclicker.stopped.store(false, Ordering::SeqCst);
    let started = autoclicker.autoclick(20, None, None);
    assert!(!started, "Autoclicker should not start if already running");
}
