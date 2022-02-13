// mod test;


use image::RgbImage;

pub struct PixelBuffer {
    pub width: u32,
    pub height: u32,
    data: Vec<u8>,
}

pub trait ToPixelBuffer {
    fn to_pixel_buffer(&self) -> PixelBuffer;
}

impl ToPixelBuffer for RgbImage {
    fn to_pixel_buffer(&self) -> PixelBuffer {
        PixelBuffer {
            width: self.width(),
            height: self.height(),
            data: self.as_raw().to_vec(),
        }
    }
}
