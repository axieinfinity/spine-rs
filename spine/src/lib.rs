pub mod animation;
pub mod atlas;
pub mod backend;
pub mod geometry;
pub mod r#impl;
pub mod skeleton;

mod error;
mod render;

mod result {
    pub type Result<T> = std::result::Result<T, super::Error>;
}

pub use error::*;
pub use geometry::*;
pub use render::*;
pub use result::*;

pub use image;
pub use spine_sys as sys;

#[cfg(feature = "glium")]
pub use glium;
