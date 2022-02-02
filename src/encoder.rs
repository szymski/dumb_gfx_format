use std::io::{Seek, SeekFrom, Write};
use image::RgbImage;

use memoffset::offset_of;
use crate::dgf::Dgf;
use crate::dgf::format::{DgfHeader, DgfVersion, ImageMode, DGF_MAGIC, DGF_DATA_SECTION_MAGIC};

pub fn encode<B: Write + Seek>(input: RgbImage, buffer: &mut B) {
    let mut dgf = Dgf::new(DgfVersion::Version1, 128, 128, ImageMode::Rgb8);
    dgf.data = input.into_raw();

    // Write header
    let header = dgf.to_dgf_header();
    let header_slice = unsafe {
        let p: *const DgfHeader = &header;
        std::slice::from_raw_parts::<u8>(p as *const u8, std::mem::size_of::<DgfHeader>())
    };
    buffer.write(header_slice).unwrap();

    // Write image data
    let data_offset = buffer.stream_position().unwrap();
    buffer.write(&dgf.data).unwrap();

    println!("Data offset: {:?}", offset_of!(DgfHeader, data_offset));

    // Update data offset
    let data_offset_pos = offset_of!(DgfHeader, data_offset);
    buffer.seek(SeekFrom::Start(data_offset_pos as u64)).unwrap();
    buffer.write(&(data_offset as u32).to_le_bytes()).unwrap();
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
            data_length: self.data.len() as u32,
            data_offset: 0xDEADBEEF,
            data: [],
        }
    }
}
