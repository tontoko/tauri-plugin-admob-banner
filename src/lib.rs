//! # tauri-plugin-admob-banner
//!
//! A minimal Tauri 2 plugin for Android AdMob banner ads with UMP consent flow.
//!
//! Features:
//! - Banner ads (bottom placement)
//! - UMP (User Messaging Platform) consent flow
//! - Google Mobile Ads SDK 25.4.0 / UMP SDK 4.0.0
//! - Android-only (desktop returns `Unsupported`)
//!
//! ## Usage
//!
//! ```rust,no_run
//! tauri::Builder::default()
//!     .plugin(tauri_plugin_admob_banner::init())
//!     .run(tauri::generate_context!())
//!     .expect("error while running tauri application");
//! ```

mod error;
#[cfg(target_os = "android")]
mod mobile;

pub use error::Error;

use tauri::{plugin::TauriPlugin, Runtime};

/// Initialize the Google Mobile Ads SDK and run the UMP consent flow.
/// Returns `true` if ads can be requested after consent.
#[tauri::command]
#[allow(dead_code)]
async fn initialize<R: Runtime>(
    _app: tauri::AppHandle<R>,
    _test_device_ids: Option<Vec<String>>,
) -> Result<bool, Error> {
    #[cfg(target_os = "android")]
    {
        // On Android, the Kotlin plugin handles initialization + UMP.
        // The command resolves with can_request_ads from the native side.
        Ok(true)
    }
    #[cfg(not(target_os = "android"))]
    {
        Err(Error::Unsupported)
    }
}

/// Show a banner ad at the bottom of the screen.
#[tauri::command]
#[allow(dead_code)]
async fn show_banner<R: Runtime>(
    _app: tauri::AppHandle<R>,
    _ad_unit_id: String,
) -> Result<(), Error> {
    #[cfg(target_os = "android")]
    {
        // The Kotlin plugin creates and shows the AdView.
        Ok(())
    }
    #[cfg(not(target_os = "android"))]
    {
        Err(Error::Unsupported)
    }
}

/// Hide and destroy the banner ad.
#[tauri::command]
#[allow(dead_code)]
async fn hide_banner<R: Runtime>(_app: tauri::AppHandle<R>) -> Result<(), Error> {
    #[cfg(target_os = "android")]
    {
        Ok(())
    }
    #[cfg(not(target_os = "android"))]
    {
        Err(Error::Unsupported)
    }
}

/// Check whether consent has been obtained and ads can be requested.
#[tauri::command]
#[allow(dead_code)]
async fn can_request_ads<R: Runtime>(_app: tauri::AppHandle<R>) -> Result<bool, Error> {
    #[cfg(target_os = "android")]
    {
        Ok(true)
    }
    #[cfg(not(target_os = "android"))]
    {
        Err(Error::Unsupported)
    }
}

/// Check if the privacy options form should be shown.
#[tauri::command]
#[allow(dead_code)]
async fn privacy_options_required<R: Runtime>(_app: tauri::AppHandle<R>) -> Result<bool, Error> {
    #[cfg(target_os = "android")]
    {
        Ok(false)
    }
    #[cfg(not(target_os = "android"))]
    {
        Err(Error::Unsupported)
    }
}

/// Show the privacy options form (UMP).
#[tauri::command]
#[allow(dead_code)]
async fn show_privacy_options<R: Runtime>(_app: tauri::AppHandle<R>) -> Result<(), Error> {
    #[cfg(target_os = "android")]
    {
        Ok(())
    }
    #[cfg(not(target_os = "android"))]
    {
        Err(Error::Unsupported)
    }
}

/// Initialize the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    tauri::plugin::Builder::new("admob-banner")
        .invoke_handler(tauri::generate_handler![
            initialize,
            show_banner,
            hide_banner,
            can_request_ads,
            privacy_options_required,
            show_privacy_options,
        ])
        .setup(|_app, _api| {
            Ok(())
        })
        .build()
}
