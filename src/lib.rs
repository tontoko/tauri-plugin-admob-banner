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
#[cfg(mobile)]
mod mobile;

pub use error::{Error, Result};

#[cfg(mobile)]
use mobile::AdmobBanner;

use tauri::{plugin::TauriPlugin, Runtime};

#[cfg(mobile)]
use tauri::Manager;

/// Initialize the Google Mobile Ads SDK and run the UMP consent flow.
/// Returns `true` if ads can be requested after consent.
#[tauri::command]
async fn initialize<R: Runtime>(
    app: tauri::AppHandle<R>,
    _test_device_ids: Option<Vec<String>>,
) -> std::result::Result<bool, Error> {
    #[cfg(mobile)]
    {
        let admob = app.state::<AdmobBanner<R>>();
        admob.initialize()
    }
    #[cfg(not(mobile))]
    {
        let _ = app;
        Err(Error::Unsupported)
    }
}

/// Show a banner ad at the bottom of the screen.
#[tauri::command]
async fn show_banner<R: Runtime>(
    app: tauri::AppHandle<R>,
    ad_unit_id: String,
) -> std::result::Result<(), Error> {
    #[cfg(mobile)]
    {
        let admob = app.state::<AdmobBanner<R>>();
        admob.show_banner(ad_unit_id)
    }
    #[cfg(not(mobile))]
    {
        let _ = (app, ad_unit_id);
        Err(Error::Unsupported)
    }
}

/// Hide and destroy the banner ad.
#[tauri::command]
async fn hide_banner<R: Runtime>(
    app: tauri::AppHandle<R>,
) -> std::result::Result<(), Error> {
    #[cfg(mobile)]
    {
        let admob = app.state::<AdmobBanner<R>>();
        admob.hide_banner()
    }
    #[cfg(not(mobile))]
    {
        let _ = app;
        Err(Error::Unsupported)
    }
}

/// Check whether consent has been obtained and ads can be requested.
#[tauri::command]
async fn can_request_ads<R: Runtime>(
    app: tauri::AppHandle<R>,
) -> std::result::Result<bool, Error> {
    #[cfg(mobile)]
    {
        let admob = app.state::<AdmobBanner<R>>();
        admob.can_request_ads()
    }
    #[cfg(not(mobile))]
    {
        let _ = app;
        Err(Error::Unsupported)
    }
}

/// Check if the privacy options form should be shown.
#[tauri::command]
async fn privacy_options_required<R: Runtime>(
    app: tauri::AppHandle<R>,
) -> std::result::Result<bool, Error> {
    #[cfg(mobile)]
    {
        let admob = app.state::<AdmobBanner<R>>();
        admob.privacy_options_required()
    }
    #[cfg(not(mobile))]
    {
        let _ = app;
        Err(Error::Unsupported)
    }
}

/// Show the privacy options form (UMP).
#[tauri::command]
async fn show_privacy_options<R: Runtime>(
    app: tauri::AppHandle<R>,
) -> std::result::Result<(), Error> {
    #[cfg(mobile)]
    {
        let admob = app.state::<AdmobBanner<R>>();
        admob.show_privacy_options()
    }
    #[cfg(not(mobile))]
    {
        let _ = app;
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
        .setup(|app, api| {
            #[cfg(mobile)]
            {
                let admob = mobile::init(app, api)?;
                app.manage(admob);
            }
            #[cfg(not(mobile))]
            {
                let _ = (app, api);
            }
            Ok(())
        })
        .build()
}
