use crate::Error;

/// Mobile plugin functions — these are thin wrappers that delegate to the
/// Tauri command layer. The actual native calls happen in the Kotlin plugin.
///
/// On non-Android platforms, all functions return `Unsupported`.

pub fn initialize_blocking(
    _test_device_ids: Vec<String>,
) -> Result<bool, Error> {
    #[cfg(target_os = "android")]
    {
        Err(Error::Admob(
            "Use the Tauri command 'initialize' instead".into(),
        ))
    }
    #[cfg(not(target_os = "android"))]
    {
        Err(Error::Unsupported)
    }
}
