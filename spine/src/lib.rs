mod atlas;
mod atlas_page;
mod error;
pub mod r#impl;
mod texture;

mod result {
    pub type Result<T> = std::result::Result<T, super::Error>;
}

pub use atlas::*;
pub use atlas_page::*;
pub use error::*;
pub use result::*;
pub use texture::*;

#[doc(hidden)]
pub use spine_sys;
