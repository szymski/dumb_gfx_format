use std::io::{Read, Seek, SeekFrom};
use std::mem::MaybeUninit;
use image::codecs::png::CompressionType::Default;
use image::RgbImage;
use crate::dgf::format::DgfHeader;

pub fn decode<B: Read + Seek>(buffer: &mut B) -> RgbImage {
    // Read header

    let mut header: DgfHeader = unsafe { MaybeUninit::uninit().assume_init() };
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
    buffer.seek(SeekFrom::Start(header.data_offset as u64)).unwrap();
    buffer.read_exact(data.as_mut_slice()).unwrap();

    RgbImage::from_raw(header.properties.width as u32, header.properties.height as u32, data).unwrap()
}
