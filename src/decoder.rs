use crate::dct::dct_reverse;
use crate::dgf::format::compression::image::{ImageCompression, ImageCompressionMode};
use crate::dgf::format::compression::post::{PostCompression, PostCompressionMode};
use crate::dgf::format::{DgfHeader, DgfImageProperties};
use crate::util::get_image_chunks;
use byteorder::{ReadBytesExt, LE};
use image::RgbImage;
use std::borrow::Borrow;
use std::io::{Cursor, Error, ErrorKind, Read, Seek, SeekFrom};
use std::mem::MaybeUninit;

pub fn decode<B: Read + Seek>(buffer: &mut B) -> Result<RgbImage, Error> {
    // Read header

    let mut header: DgfHeader = unsafe { MaybeUninit::assume_init(MaybeUninit::uninit()) };
    let header_slice = unsafe {
        let p: *const DgfHeader = &mut header;
        std::slice::from_raw_parts_mut::<u8>(p as *mut u8, std::mem::size_of::<DgfHeader>())
    };
    buffer.read_exact(header_slice).unwrap();

    println!("Version: {}", header.version as u16);
    println!("Data offset: {}", header.data_offset);
    println!("Data length: {}", header.data_length);

    // Read image data
    let mut data = vec![0u8; header.data_length as usize];
    buffer
        .seek(SeekFrom::Start(header.data_offset as u64))
        .unwrap();
    buffer.read_exact(data.as_mut_slice()).unwrap();

    // Decompress
    let decompressed_data = post_decompress(data, header.compression.post)?;
    println!("Decompressed data length: {}", decompressed_data.len());

    let decompressed = decompress_image(
        &decompressed_data,
        header.properties,
        header.compression.image,
    );

    // Create RgbImage
    let image = RgbImage::from_raw(
        header.properties.width as u32,
        header.properties.height as u32,
        decompressed,
    )
    .ok_or_else(|| Error::new(ErrorKind::Other, "Failed to create RgbImage from raw bytes"))?;

    Ok(image)
}

pub fn post_decompress(
    compressed: Vec<u8>,
    compression: PostCompression,
) -> std::io::Result<Vec<u8>> {
    match compression.mode {
        PostCompressionMode::None => Ok(compressed),
        PostCompressionMode::Snappy => {
            let mut reader = snap::read::FrameDecoder::new(compressed.as_slice());
            let mut data = Vec::new();
            reader.read_to_end(&mut data).unwrap();
            Ok(data)
        }
        _ => Err(Error::new(
            ErrorKind::Other,
            format!("Unsupported compression mode ({})", compression.mode as u8),
        )),
    }
}

pub fn decompress_image(
    buffer: &Vec<u8>,
    properties: DgfImageProperties,
    compression: ImageCompression,
) -> Vec<u8> {
    match compression.mode {
        ImageCompressionMode::None => buffer.clone(),
        ImageCompressionMode::Dct => {
            let chunks = get_image_chunks(
                properties.width as u32,
                properties.height as u32,
                compression.dct.block_size as u32,
            );

            println!("DCT total signal length (f32): {}", buffer.len() / 4);
            println!("DCT: {:?}", compression.dct);
            println!("DCT block count: {}", chunks.len());

            let mut signal_cursor = Cursor::new(buffer);

            let chunks_signal = chunks
                .iter()
                .map(|chunk|
                    (chunk, {
                        let mut nums = vec![0f32; compression.dct.coefficient_count as usize];
                        signal_cursor.read_f32_into::<LE>(&mut nums).unwrap();
                        nums
                    })
                );

            let image_buffer_size = properties.width as usize * properties.height as usize * 3;
            let mut image_buffer = vec![0u8; image_buffer_size];

            for (chunk, dct_coefficients) in chunks_signal {
                // println!("Chunk: {:?}", dct_coefficients);

                for y in chunk.range_y() {
                    for x in chunk.range_x() {
                        let pixel_index = ((y * properties.width as u32 + x) * 3) as usize;
                        let (chunk_x, chunk_y) = (x - chunk.x, y - chunk.y);

                        let pixel = dct_reverse(&dct_coefficients, chunk.size, chunk_x, chunk_y);

                        image_buffer[pixel_index + 0] = (pixel * 255.0) as u8;
                        image_buffer[pixel_index + 1] = (pixel * 255.0) as u8;
                        image_buffer[pixel_index + 2] = (pixel * 255.0) as u8;
                    }
                }
            }

            image_buffer
        }
        _ => {
            panic!("Unsupported compression mode ({:?})", compression.mode);
        }
    }
}
