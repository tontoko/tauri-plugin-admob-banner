const COMMANDS: &[&str] = &[
    "initialize",
    "show_banner",
    "hide_banner",
    "can_request_ads",
    "privacy_options_required",
    "show_privacy_options",
];

fn main() {
    tauri_plugin::Builder::new(COMMANDS)
        .android_path("android")
        .build();
}
