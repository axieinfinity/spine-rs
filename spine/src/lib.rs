mod animation_state;
mod animation_state_data;
mod atlas;
mod atlas_page;
mod error;
pub mod r#impl;
mod skeleton;
mod skeleton_data;
mod skeleton_json;
mod texture;

mod result {
    pub type Result<T> = std::result::Result<T, super::Error>;
}

pub use animation_state::*;
pub use animation_state_data::*;
pub use atlas::*;
pub use atlas_page::*;
pub use error::*;
pub use result::*;
pub use skeleton::*;
pub use skeleton_data::*;
pub use skeleton_json::*;
pub use texture::*;

#[doc(hidden)]
pub use spine_sys;
