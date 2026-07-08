use serde::de::DeserializeOwned;
use serde::Serialize;
use tauri::{
    plugin::{PluginApi, PluginHandle},
    AppHandle, Runtime,
};

#[cfg(target_os = "android")]
const PLUGIN_IDENTIFIER: &str = "com.tontoko.admob_banner";

/// Android plugin handle — wraps the Kotlin `AdmobBannerPlugin`.
pub struct AdmobBanner<R: Runtime>(pub PluginHandle<R>);

/// Arguments for the `show_banner` Kotlin command.
/// Field names must match the Kotlin `@InvokeArg` class (camelCase).
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ShowBannerArgs {
    ad_unit_id: String,
}

/// Initializes the Kotlin plugin class on Android.
pub fn init<R: Runtime, C: DeserializeOwned>(
    _app: &AppHandle<R>,
    api: PluginApi<R, C>,
) -> crate::Result<AdmobBanner<R>> {
    #[cfg(target_os = "android")]
    {
        let handle = api.register_android_plugin(PLUGIN_IDENTIFIER, "AdmobBannerPlugin")?;
        Ok(AdmobBanner(handle))
    }
    #[cfg(not(target_os = "android"))]
    {
        let _ = api;
        Err(crate::Error::Unsupported)
    }
}

impl<R: Runtime> AdmobBanner<R> {
    pub fn initialize(&self) -> crate::Result<bool> {
        let v = self
            .0
            .run_mobile_plugin::<serde_json::Value>("initialize", ())?;
        v.get("can_request_ads")
            .and_then(|b| b.as_bool())
            .ok_or_else(|| crate::Error::Admob("missing can_request_ads in response".into()))
    }

    pub fn show_banner(&self, ad_unit_id: String) -> crate::Result<()> {
        let args = ShowBannerArgs { ad_unit_id };
        self.0.run_mobile_plugin::<()>("show_banner", args)?;
        Ok(())
    }

    pub fn hide_banner(&self) -> crate::Result<()> {
        self.0.run_mobile_plugin::<()>("hide_banner", ())?;
        Ok(())
    }

    pub fn can_request_ads(&self) -> crate::Result<bool> {
        let v = self
            .0
            .run_mobile_plugin::<serde_json::Value>("can_request_ads", ())?;
        v.get("value")
            .and_then(|b| b.as_bool())
            .ok_or_else(|| crate::Error::Admob("missing value in response".into()))
    }

    pub fn privacy_options_required(&self) -> crate::Result<bool> {
        let v = self
            .0
            .run_mobile_plugin::<serde_json::Value>("privacy_options_required", ())?;
        v.get("value")
            .and_then(|b| b.as_bool())
            .ok_or_else(|| crate::Error::Admob("missing value in response".into()))
    }

    pub fn show_privacy_options(&self) -> crate::Result<()> {
        self.0.run_mobile_plugin::<()>("show_privacy_options", ())?;
        Ok(())
    }
}
