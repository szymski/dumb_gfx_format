mod pixel_buffer;
mod encoder;
mod decoder;
mod dgf;

use std::borrow::{Borrow, BorrowMut};
use std::fmt::{Formatter, Write};
use std::fs::File;
use image::{GenericImageView, ImageBuffer, Pixel, Rgb, RgbImage};
use image::buffer::Pixels;
use image::imageops::FilterType;
use crate::decoder::decode;
use crate::pixel_buffer::{PixelBuffer, ToPixelBuffer};
use crate::encoder::encode;

const SIZE: i32 = 128;

fn main() {
    println!("dumb gfx format v0.1");

    // println!("{}", std::mem::size_of::<DgfHeader>());

    const TO_ENCODE_PATH: &str = "lena.png";
    const ENCODED_PATH: &str = "output.dgf";
    const DECODED_PATH: &str = "decoded.png";

    {
        let input = load_image(TO_ENCODE_PATH);

        println!("Encoding '{}' file...", TO_ENCODE_PATH);

        let mut file = File::create(ENCODED_PATH).unwrap();
        encode(input, &mut file);

        println!("Saved to '{}'", ENCODED_PATH);
    }

    {
        let mut input = File::open(ENCODED_PATH).unwrap();

        println!("Decoding '{}' file...", ENCODED_PATH);

        let decoded = decode(&mut input);
        decoded.save_with_format(DECODED_PATH, image::ImageFormat::Png).unwrap();

        println!("Saved decoded file to '{}'", DECODED_PATH);
    }

    return;

    let mut img = image::RgbImage::new(SIZE as u32, SIZE as u32);
    let mut buffer: PixelBuffer = img.to_pixel_buffer();
    buffer.width = 128;

    for x in 0..img.width() {
        for y in 0..img.height() {
            // let rgb = input.get_pixel(x, img.height() - y - 1).to_rgb();
            // img.put_pixel(x, y, Rgb([rgb[0], rgb[1], rgb[2]]));
        }
    }

    img.save("image.png").unwrap();

    std::process::Command::new("explorer.exe")
        .arg("image.png")
        .output()
        .unwrap();

    println!("Saved to image.png");
}

fn load_image(path: &str) -> RgbImage {
    image::open(path).unwrap().resize(128, 128, FilterType::Gaussian).to_rgb8()
}

fn get_pixel_screen(x: u32, y: u32) -> Rgb<u8> {
    let fx = (x as i32 - SIZE / 2) as f32 / (SIZE as f32) * 3.0;
    let fy = (y as i32 - SIZE / 2) as f32 / (SIZE as f32) * 3.0;
    get_pixel(fx, fy)
}

fn get_pixel(x: f32, y: f32) -> Rgb<u8> {
    let mut z = (x, y);
    let c = z;

    const ITERATIONS: u32 = 256;

    let mut i = 0;
    for _ in 0..ITERATIONS {
        i += 1;

        z = (z.0 * z.0 - z.1 * z.1 + c.0, 2.0 * z.0 * z.1 + c.1);

        let len = z.0 * z.0 + z.1 * z.1;
        if len > 4.0 {
            return Rgb([i as u8, 0, 0]);
        }
    }

    Rgb([0, 0, 0])
    // Rgb([(x * 255.0) as u8, (y * 255.0) as u8, 0])
}

// impl std::fmt::Display for (f32, f32) {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         f.write_fmt("asd", self.0);
//     }
// }
