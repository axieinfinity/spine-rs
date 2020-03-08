use std::error::Error;

use image::DynamicImage;
use image::GenericImageView;

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
        .map_err(|error| (Box::new(error) as Box<dyn Error>).into())
}

spine::impl_spine!(DynamicImage, read_texture_file);
