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

use tauri::{plugin::TauriPlugin, Manager, Runtime};

/// Commands exposed to the JS side.
#[tauri::command]
#[allow(dead_code)]
async fn initialize<R: Runtime>(
    app: tauri::AppHandle<R>,
    test_device_ids: Vec<String>,
) -> Result<bool, Error> {
    #[cfg(target_os = "android")]
    {
        let (tx, rx) = std::sync::mpsc::channel();
        mobile::initialize(&app, test_device_ids, move |can_request| {
            let _ = tx.send(can_request);
        })?;
        let can_request = rx.recv().unwrap_or(false);
        Ok(can_request)
    }
    #[cfg(not(target_os = "android"))]
    {
        let _ = (app, test_device_ids);
        Err(Error::Unsupported)
    }
}

#[tauri::command]
#[allow(dead_code)]
async fn show_banner<R: Runtime>(
    app: tauri::AppHandle<R>,
    ad_unit_id: String,
) -> Result<(), Error> {
    #[cfg(target_os = "android")]
    {
        mobile::show_banner(&app, ad_unit_id)?;
        Ok(())
    }
    #[cfg(not(target_os = "android"))]
    {
        let _ = (app, ad_unit_id);
        Err(Error::Unsupported)
    }
}

#[tauri::command]
#[allow(dead_code)]
async fn hide_banner<R: Runtime>(app: tauri::AppHandle<R>) -> Result<(), Error> {
    #[cfg(target_os = "android")]
    {
        mobile::hide_banner(&app)?;
        Ok(())
    }
    #[cfg(not(target_os = "android"))]
    {
        let _ = app;
        Err(Error::Unsupported)
    }
}

/// Initialize the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    tauri::plugin::Builder::new("tauri-plugin-admob-banner")
        .invoke_handler(tauri::generate_handler![
            initialize,
            show_banner,
            hide_banner,
        ])
        .setup(|app, _api| {
            #[cfg(target_os = "android")]
            {
                let _ = app;
            }
            Ok(())
        })
        .build()
}
