use std::{ffi::CString, path::Path};

use super::{error::Error, result::Result};

pub fn c_path(path: impl AsRef<Path>) -> Result<CString> {
    let path = path
        .as_ref()
        .to_str()
        .ok_or(Error::invalid_data("path is not in valid UTF-8"))?;

    Ok(CString::new(path)?)
}
