pub mod animation;
pub mod atlas;
pub mod attachment;
pub mod r#impl;
pub mod skeleton;
pub mod slot;

#[cfg(feature = "glium")]
pub mod glium;

mod error;
mod render;
mod texture;

mod result {
    pub type Result<T> = std::result::Result<T, super::Error>;
}

pub use error::*;
pub use render::*;
pub use result::*;
pub use texture::*;

#[doc(hidden)]
pub use spine_sys;
