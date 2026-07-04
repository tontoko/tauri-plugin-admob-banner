use crate::Error;
use tauri::{AppHandle, Runtime, plugin::PluginHandle};

#[cfg(target_os = "android")]
const PLUGIN_IDENTIFIER: &str = "tauri-plugin-admob-banner";

#[cfg(target_os = "android")]
fn plugin_handle<R: Runtime>(
    app: &AppHandle<R>,
) -> Result<PluginHandle<R>, Error> {
    app.plugin(PLUGIN_IDENTIFIER)
        .map_err(|_| Error::NotInitialized)
}

/// Initialize the Google Mobile Ads SDK and run the UMP consent flow.
/// Must be called once at app startup before requesting ads.
#[cfg(target_os = "android")]
pub fn initialize<R: Runtime, F: FnOnce(bool) + Send + 'static>(
    app: &AppHandle<R>,
    test_device_ids: Vec<String>,
    callback: F,
) -> Result<(), Error> {
    let handle = plugin_handle(app)?;
    handle
        .run_mobile_plugin("initialize", move |data| {
            let can_request = data
                .get_bool("can_request_ads")
                .unwrap_or(false);
            callback(can_request);
        })
        .map_err(|e| Error::Admob(e.to_string()))?;
    // Pass test_device_ids via the plugin's init config
    let _ = test_device_ids;
    Ok(())
}

#[cfg(not(target_os = "android"))]
pub fn initialize<R: Runtime, F: FnOnce(bool) + Send + 'static>(
    _app: &AppHandle<R>,
    _test_device_ids: Vec<String>,
    _callback: F,
) -> Result<(), Error> {
    Err(Error::Unsupported)
}

/// Show a banner ad at the bottom of the screen.
#[cfg(target_os = "android")]
pub fn show_banner<R: Runtime>(
    app: &AppHandle<R>,
    ad_unit_id: String,
) -> Result<(), Error> {
    let handle = plugin_handle(app)?;
    handle
        .run_mobile_plugin("show_banner", |_| {})
        .map_err(|e| Error::Admob(e.to_string()))?;
    let _ = ad_unit_id;
    Ok(())
}

#[cfg(not(target_os = "android"))]
pub fn show_banner<R: Runtime>(
    _app: &AppHandle<R>,
    _ad_unit_id: String,
) -> Result<(), Error> {
    Err(Error::Unsupported)
}

/// Hide and destroy the banner ad.
#[cfg(target_os = "android")]
pub fn hide_banner<R: Runtime>(app: &AppHandle<R>) -> Result<(), Error> {
    let handle = plugin_handle(app)?;
    handle
        .run_mobile_plugin("hide_banner", |_| {})
        .map_err(|e| Error::Admob(e.to_string()))?;
    Ok(())
}

#[cfg(not(target_os = "android"))]
pub fn hide_banner<R: Runtime>(_app: &AppHandle<R>) -> Result<(), Error> {
    Err(Error::Unsupported)
}

/// Check whether consent has been obtained and ads can be requested.
#[cfg(target_os = "android")]
pub fn can_request_ads<R: Runtime, F: FnOnce(bool) + Send + 'static>(
    app: &AppHandle<R>,
    callback: F,
) -> Result<(), Error> {
    let handle = plugin_handle(app)?;
    handle
        .run_mobile_plugin("can_request_ads", move |data| {
            let can = data.get_bool("value").unwrap_or(false);
            callback(can);
        })
        .map_err(|e| Error::Admob(e.to_string()))?;
    Ok(())
}

#[cfg(not(target_os = "android"))]
pub fn can_request_ads<R: Runtime, F: FnOnce(bool) + Send + 'static>(
    _app: &AppHandle<R>,
    _callback: F,
) -> Result<(), Error> {
    Err(Error::Unsupported)
}

/// Check if the privacy options form should be shown.
#[cfg(target_os = "android")]
pub fn privacy_options_required<R: Runtime, F: FnOnce(bool) + Send + 'static>(
    app: &AppHandle<R>,
    callback: F,
) -> Result<(), Error> {
    let handle = plugin_handle(app)?;
    handle
        .run_mobile_plugin("privacy_options_required", move |data| {
            let required = data.get_bool("value").unwrap_or(false);
            callback(required);
        })
        .map_err(|e| Error::Admob(e.to_string()))?;
    Ok(())
}

#[cfg(not(target_os = "android"))]
pub fn privacy_options_required<R: Runtime, F: FnOnce(bool) + Send + 'static>(
    _app: &AppHandle<R>,
    _callback: F,
) -> Result<(), Error> {
    Err(Error::Unsupported)
}

/// Show the privacy options form (UMP).
#[cfg(target_os = "android")]
pub fn show_privacy_options<R: Runtime, F: FnOnce() + Send + 'static>(
    app: &AppHandle<R>,
    callback: F,
) -> Result<(), Error> {
    let handle = plugin_handle(app)?;
    handle
        .run_mobile_plugin("show_privacy_options", move |_| {
            callback();
        })
        .map_err(|e| Error::Admob(e.to_string()))?;
    Ok(())
}

#[cfg(not(target_os = "android"))]
pub fn show_privacy_options<R: Runtime, F: FnOnce() + Send + 'static>(
    _app: &AppHandle<R>,
    _callback: F,
) -> Result<(), Error> {
    Err(Error::Unsupported)
}
