use ashpd::Error;
use ashpd::desktop::background::Background;

/// Check whether the portal will start the application automatically.
pub async fn is_autostart_enabled() -> bool {
    let response = Background::request().send().await;
    let response = match response {
        Ok(response) => response.response(),
        Err(_) => return false,
    };
    let response = match response {
        Ok(response) => response,
        Err(_) => return false,
    };

    response.auto_start()
}

/// Request that the portal starts the application automatically.
pub async fn enable_autostart() -> Result<bool, Error> {
    set_autostart(true).await
}

/// Request that the portal stops starting the application automatically.
pub async fn disable_autostart() -> Result<bool, Error> {
    set_autostart(false).await
}

async fn set_autostart(enable: bool) -> Result<bool, Error> {
    let response = Background::request()
        .auto_start(enable)
        .send()
        .await?
        .response()?;

    Ok(response.auto_start())
}
