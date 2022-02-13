use image::RgbImage;
use std::borrow::{Borrow};
use std::io::{Error, ErrorKind, Seek, SeekFrom, Write};

use crate::dgf::format::compression::image::{ImageCompression, ImageCompressionMode};
use crate::dgf::format::compression::post::{PostCompression, PostCompressionMode};
use crate::dgf::format::{DgfHeader, DgfVersion, ImageMode, DGF_DATA_SECTION_MAGIC, DGF_MAGIC, DgfImageProperties};
use crate::dgf::Dgf;
use crate::util::get_image_chunks;
use memoffset::offset_of;
use crate::dct::dct;

pub fn encode<B: Write + Seek>(input: RgbImage, _buffer: &mut B) -> Result<(), Error> {
    let mut buffer = _buffer;

    let mut dgf = Dgf::new(
        DgfVersion::Version1,
        input.width() as u16,
        input.height() as u16,
        ImageMode::Rgb8,
    );
    dgf.properties.mode = ImageMode::Grayscale8;
    dgf.compression.image.mode = ImageCompressionMode::Dct;
    dgf.compression.image.dct.coefficient_count = 64;
    dgf.compression.image.dct.block_size = 8;
    dgf.compression.post.mode = PostCompressionMode::Snappy;

    // Compress image
    let compressed = compress_image(&input, dgf.properties, dgf.compression.image);
    // let compressed = vec![0u8];

    // Write header
    let header = dgf.to_dgf_header();
    let header_slice = unsafe {
        let p: *const DgfHeader = &header;
        std::slice::from_raw_parts::<u8>(p as *const u8, std::mem::size_of::<DgfHeader>())
    };
    buffer.write_all(header_slice)?;

    // Write post compressed image data

    // TODO: Support different channel configs

    let data_offset = buffer.stream_position()?;

    apply_post_compression(&compressed, dgf.compression.post, &mut buffer)?;

    // Calculate compressed data length
    let data_end_offset = buffer.stream_position()?;
    let data_length = data_end_offset - data_offset;

    println!("Data offset: {:?}", offset_of!(DgfHeader, data_offset));
    println!("Data length (raw): {}", input.borrow().as_raw().len());
    println!("Data length (compressed): {}", compressed.len());
    println!("Data length (post-compressed): {:?}", data_length);

    // Update data length
    let data_length_pos = offset_of!(DgfHeader, data_length);
    buffer.seek(SeekFrom::Start(data_length_pos as u64))?;
    buffer.write_all(&(data_length as u32).to_le_bytes())?;

    // Update data offset
    let data_offset_pos = offset_of!(DgfHeader, data_offset);
    buffer.seek(SeekFrom::Start(data_offset_pos as u64))?;
    buffer.write_all(&(data_offset as u32).to_le_bytes())?;

    Ok(())
}

pub fn compress_image(image: &RgbImage, properties: DgfImageProperties, compression: ImageCompression) -> Vec<u8> {
    println!("Compressing image with mode '{:?}'", compression.mode);

    match properties.mode {
        ImageMode::Grayscale8 => {
            println!("Using RGB8 mode");
        },
        _ => panic!("Unsupported image mode '{:?}'", properties.mode),
    }

    match compression.mode {
        ImageCompressionMode::None => image.as_raw().clone(),
        ImageCompressionMode::Dct => {
            println!("DCT: {:?}", compression.dct);

            let chunks = get_image_chunks(
                image.width(),
                image.height(),
                compression.dct.block_size as u32,
            );

            let mut chunk_signals = Vec::<f32>::new();

            for chunk in chunks {
                let mut chunk_signal = Vec::<f32>::new();

                for y in chunk.range_y() {
                    for x in chunk.range_x() {
                        let [r, g, b] = image.get_pixel(x, y).0;
                        chunk_signal.push(if x < image.width() && y < image.height() {
                            r as f32 / 255.0
                        } else {
                            0.0
                        });
                    }
                }

                let chunk_dct_signal = &mut (0..compression.dct.coefficient_count)
                    .map(|k| dct(&chunk_signal, k as u32))
                    .collect::<Vec<f32>>();

                chunk_signals.append(chunk_dct_signal);
            }

            let raw = unsafe {
                std::slice::from_raw_parts(chunk_signals.as_ptr() as *const u8, chunk_signals.len() * 4)
            };

            Vec::from(raw)
        }
        _ => {
            panic!("Unsupported compression mode ({:?})", compression.mode);
        }
    }
}

pub fn apply_post_compression<T: Write>(
    data: &[u8],
    compression: PostCompression,
    buffer: &mut T,
) -> std::io::Result<()> {
    println!("Post compressing with mode '{:?}'", compression.mode);

    match compression.mode {
        PostCompressionMode::None => buffer.write_all(data),
        PostCompressionMode::Snappy => {
            let mut encoding_writer = snap::write::FrameEncoder::new(buffer);
            encoding_writer.write_all(data)
        }
        _ => Err(Error::new(
            ErrorKind::Other,
            format!("Unsupported compression mode ({})", compression.mode as u8),
        )),
    }
}

trait ToDgfHeader {
    fn to_dgf_header(&self) -> DgfHeader;
}

impl ToDgfHeader for Dgf {
    fn to_dgf_header(&self) -> DgfHeader {
        DgfHeader {
            magic: DGF_MAGIC,
            version: self.version,
            reserved: 0,
            properties: self.properties,
            compression: self.compression,
            data_section_magic: DGF_DATA_SECTION_MAGIC,
            data_length: 0xDEADBEEF,
            data_offset: 0xDEADBEEF,
            data: [],
        }
    }
}
