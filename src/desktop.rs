use crate::Error;

pub fn initialize<F: FnOnce(bool) + Send + 'static>(
    _test_device_ids: Vec<String>,
    _callback: F,
) -> Result<(), Error> {
    Err(Error::Unsupported)
}

pub fn show_banner(_ad_unit_id: String) -> Result<(), Error> {
    Err(Error::Unsupported)
}

pub fn hide_banner() -> Result<(), Error> {
    Err(Error::Unsupported)
}
