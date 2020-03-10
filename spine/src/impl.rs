use std::{
    ffi::{CStr, CString},
    fs::File,
    io::{BufReader, Read},
    os::raw::{c_char, c_int, c_void},
};

use image::{DynamicImage, GenericImageView};
use spine_sys::spAtlasPage;

use super::{error::Error, result::Result};

#[no_mangle]
pub extern "C" fn _spUtil_readFile(path: *const c_char, length: *mut c_int) -> *const c_char {
    #[inline]
    fn read_text_file(path: *const c_char) -> Result<CString> {
        let path = to_str(path)?;

        let file = File::open(path)?;
        let mut reader = BufReader::new(file);

        let mut bytes = Vec::new();

        reader.read_to_end(&mut bytes)?;

        let text = CString::new(bytes)?;

        Ok(text)
    }

    let text = match read_text_file(path) {
        Ok(text) => text,
        Err(error) => {
            eprintln!("{}", error);
            return std::ptr::null();
        }
    };

    unsafe {
        *length = text.to_bytes().len() as c_int;
        text.into_raw()
    }
}

#[no_mangle]
pub extern "C" fn _spAtlasPage_createTexture(page: *mut spAtlasPage, path: *const c_char) {
    #[inline]
    fn read_texture_file(path: *const c_char) -> Result<DynamicImage> {
        let path = to_str(path)?;
        image::open(path).map_err(Error::invalid_data)
    }

    let texture = read_texture_file(path).unwrap();
    let (width, height) = texture.dimensions();

    unsafe {
        (*page).width = width as c_int;
        (*page).height = height as c_int;
        (*page).rendererObject = Box::into_raw(Box::new(texture)) as *mut c_void;
    }
}

#[no_mangle]
pub extern "C" fn _spAtlasPage_disposeTexture(page: *mut spAtlasPage) {
    unsafe {
        Box::from_raw((*page).rendererObject as *mut DynamicImage);
    }
}

#[inline]
fn to_str<'a>(s: *const c_char) -> Result<&'a str> {
    let s = unsafe { CStr::from_ptr(s) }
        .to_str()
        .map_err(Error::invalid_input)?;

    Ok(s)
}
