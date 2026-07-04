# tauri-plugin-admob-banner

A minimal **Tauri 2** plugin for **Android AdMob banner ads** with **UMP consent flow**.

## Features

- 🏷️ Banner ads (bottom placement)
- 🔒 UMP (User Messaging Platform) GDPR consent flow
- 📦 Google Mobile Ads SDK 25.4.0 / UMP SDK 4.0.0
- 🤖 Android-only (desktop/iOS returns `Unsupported`)

## Why?

Existing Tauri AdMob plugins either:
- Ship outdated GMA/UMP SDK versions
- Include unnecessary ad formats (interstitial, rewarded) when you only need banners
- Lack proper UMP consent flow for Google Play compliance

This plugin is banner-only, always uses the latest GMA/UMP SDK, and has the UMP consent flow built in.

## Installation

### Rust

```toml
# Cargo.toml
[dependencies]
tauri-plugin-admob-banner = "0.1"
```

### Tauri Builder

```rust
tauri::Builder::default()
    .plugin(tauri_plugin_admob_banner::init())
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
```

### Android Manifest

Add your AdMob App ID to `AndroidManifest.xml`:

```xml
<meta-data
    android:name="com.google.android.gms.ads.APPLICATION_ID"
    android:value="ca-app-pub-XXXXXXXXXXXXXXXX~XXXXXXXXXX"/>
```

## Usage

### Initialize (with UMP consent)

Call once at startup. This runs the UMP consent flow and initializes the GMA SDK.

```javascript
import { invoke } from '@tauri-apps/api/core';

const canRequestAds = await invoke('plugin:admob-banner|initialize');
// canRequestAds: boolean — whether consent was obtained
```

### Show Banner

```javascript
// Use Google's test ad unit ID during development
await invoke('plugin:admob-banner|show_banner', {
  adUnitId: 'ca-app-pub-3940256099942544/9214589741'
});
```

### Hide Banner

```javascript
await invoke('plugin:admob-banner|hide_banner');
```

### Check Consent Status

```javascript
const canRequest = await invoke('plugin:admob-banner|can_request_ads');
const privacyRequired = await invoke('plugin:admob-banner|privacy_options_required');

if (privacyRequired) {
  await invoke('plugin:admob-banner|show_privacy_options');
}
```

## Permissions

| Permission | Required | Purpose |
|---|---|---|
| `INTERNET` | Yes | Ad loading |
| `ACCESS_NETWORK_STATE` | Yes | Ad loading |
| `AD_ID` | Auto-merged by GMA SDK | Advertising ID for ad personalization |

## License

MIT
