use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::{BufReader, Read};
use std::os::raw::c_char;

use super::{result::Result, texture::Texture};

fn to_str<'a>(s: *const c_char) -> Result<&'a str> {
    let s = unsafe { CStr::from_ptr(s) }.to_str()?;
    Ok(s)
}

#[doc(hidden)]
pub fn read_text_file(path: *const c_char) -> Result<CString> {
    let path = to_str(path)?;

    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    let mut bytes = Vec::new();

    reader.read_to_end(&mut bytes)?;

    let text = CString::new(bytes)?;

    Ok(text)
}

#[doc(hidden)]
pub fn read_texture_file<F, T>(path: *const c_char, read: F) -> Result<T>
where
    F: Fn(&str) -> Result<T>,
    T: Texture,
{
    let path = to_str(path)?;
    let texture = read(path)?;
    Ok(texture)
}

#[macro_export]
macro_rules! impl_spine {
    ($Texture:ident, $read_texture_file:ident) => {
        pub mod spine_impls {
            use std::os::raw::{c_char, c_int, c_void};

            use $crate::{spine_sys::spAtlasPage, Texture};

            #[no_mangle]
            pub extern "C" fn _spUtil_readFile(
                path: *const c_char,
                length: *mut c_int,
            ) -> *const c_char {
                let text = match $crate::r#impl::read_text_file(path) {
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
            pub extern "C" fn _spAtlasPage_createTexture(
                page: *mut spAtlasPage,
                path: *const c_char,
            ) {
                let texture =
                    $crate::r#impl::read_texture_file(path, super::$read_texture_file).unwrap();

                unsafe {
                    (*page).width = texture.width() as c_int;
                    (*page).height = texture.height() as c_int;
                    (*page).rendererObject = Box::into_raw(Box::new(texture)) as *mut c_void;
                }
            }

            #[no_mangle]
            pub extern "C" fn _spAtlasPage_disposeTexture(page: *mut spAtlasPage) {
                unsafe {
                    Box::from_raw((*page).rendererObject as *mut super::$Texture);
                }
            }
        }
    };
}
