use ashpd::Error;
use ashpd::desktop::Session;
use ashpd::desktop::global_shortcuts::{Activated, GlobalShortcuts, NewShortcut};
use futures_util::Stream;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Wrapper around GlobalShortcuts
#[derive(Clone)]
pub struct HotkeyPortal {
    portal: Arc<Mutex<GlobalShortcuts<'static>>>,
    session: Arc<Mutex<Session<'static, GlobalShortcuts<'static>>>>,
}

impl HotkeyPortal {
    /// Register a global hotkey to trigger the autoclicker.
    pub async fn register() -> Result<Self, Error> {
        let portal = GlobalShortcuts::new().await?;
        let session = portal.create_session().await?;
        let hotkey = NewShortcut::new("Turbo Clicker Trigger", "Start/stop the autoclicker")
            .preferred_trigger("CTRL+SHIFT+F12");
        portal.bind_shortcuts(&session, &[hotkey], None).await?;
        Ok(Self {
            portal: Arc::new(Mutex::new(portal)),
            session: Arc::new(Mutex::new(session)),
        })
    }
    /// Return a stream of Activated events when the hotkey is pressed.
    pub async fn activated_stream(&self) -> Result<impl Stream<Item = Activated> + use<>, Error> {
        let portal = self.portal.lock().await;
        portal.receive_activated().await
    }
    /// Open dialog to configure the hotkey.
    pub async fn configure_hotkey(&self) {
        let portal = self.portal.lock().await;
        let session = self.session.lock().await;
        match portal.configure_shortcuts(&session, None, None).await {
            Ok(_) => (),
            Err(e) => eprintln!("Failed to open hotkey configuration dialog: {e}"),
        };
    }
}
