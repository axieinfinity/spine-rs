pub mod animation;
pub mod atlas;
pub mod backend;
pub mod geometry;
pub mod r#impl;
pub mod render;
pub mod skeleton;

mod error;
mod util;

mod result {
    pub type Result<T> = std::result::Result<T, super::error::Error>;
}

pub use error::*;
pub use result::*;

pub use image;
pub use spine_sys as sys;

#[cfg(feature = "glium")]
pub use glium;
