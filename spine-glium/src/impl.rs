use std::error::Error;

use image::{DynamicImage, GenericImageView};

#[repr(transparent)]
struct Texture(DynamicImage);

impl spine::Texture for Texture {
    #[inline]
    fn width(&self) -> u32 {
        self.0.dimensions().0
    }

    #[inline]
    fn height(&self) -> u32 {
        self.0.dimensions().0
    }
}

fn read_texture_file(path: &str) -> spine::Result<Texture> {
    image::open(path)
        .map(Texture)
        .map_err(|error| spine::Error::Other(Box::new(error) as Box<dyn Error>))
}

spine::impl_spine!(DynamicImage, read_texture_file);
