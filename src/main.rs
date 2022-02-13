mod dct;
mod decoder;
mod dgf;
mod encoder;
mod pixel_buffer;
mod util;

use crate::decoder::decode;
use crate::encoder::encode;
use crate::pixel_buffer::{ToPixelBuffer};
use image::imageops::FilterType;
use image::{ImageResult, Rgb, RgbImage};
use std::borrow::{Borrow};
use std::fs::File;
use std::io::{Error, ErrorKind};

fn main() -> Result<(), Error> {
    println!("dumb gfx format v0.1");

    const TO_ENCODE_PATH: &str = "data/lena2.png";
    const ENCODED_PATH: &str = "data/output.dgf";
    const DECODED_PATH: &str = "data/decoded.png";

    {
        let input = load_image(TO_ENCODE_PATH).unwrap();
        input
            .borrow()
            .save(TO_ENCODE_PATH.to_owned() + ".resized.png")
            .unwrap();

        println!("Encoding '{}' file...", TO_ENCODE_PATH);

        let mut file = File::create(ENCODED_PATH)?;
        encode(input, &mut file)?;

        println!("Saved to '{}'", ENCODED_PATH);
    }

    {
        let mut input = File::open(ENCODED_PATH).unwrap();

        println!("Decoding '{}' file...", ENCODED_PATH);

        let decoded = decode(&mut input)?;
        decoded
            .save_with_format(DECODED_PATH, image::ImageFormat::Png)
            .map_err(|_| Error::new(ErrorKind::Other, "Failed to save decoded image"))?;

        println!("Saved decoded file to '{}'", DECODED_PATH);
    }

    Ok(())
}

fn load_image(path: &str) -> ImageResult<RgbImage> {
    let img = image::open(path)?;
    Ok(img.resize(128, 128, FilterType::Gaussian).to_rgb8())
    // Ok(img.to_rgb8())
}
