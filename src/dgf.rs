use crate::dgf::format::{DgfImageProperties, DgfVersion, ImageMode};
use crate::dgf::format::compression::CompressionInfo;
use crate::dgf::format::compression::image::{DctProperties, ImageCompressionMode, ImageCompressionProps};
use crate::dgf::format::compression::post::{PostCompression, PostCompressionMode};

pub struct Dgf {
    pub version: DgfVersion,
    pub properties: DgfImageProperties,
    pub compression: CompressionInfo,
    pub data: Vec<u8>,
}

impl Dgf {
    pub fn new(version: DgfVersion, width: u16, height: u16, mode: ImageMode) -> Dgf {
        Dgf {
            version,
            properties: DgfImageProperties {
                mode: ImageMode::Rgb8,
                width,
                height,
                reserved: [0; 32],
            },
            compression: CompressionInfo {
                image: ImageCompressionProps {
                    mode: ImageCompressionMode::None,
                    dct: DctProperties {
                        block_size: 8,
                    },
                },
                post: PostCompression {
                    mode: PostCompressionMode::None,
                    reserved: [0; 3],
                },
            },
            data: vec![],
        }
    }
}

pub mod format {
    use crate::dgf::format::compression::CompressionInfo;

    pub const DGF_MAGIC: [u8; 4] = [7, b'D', b'G', b'F'];
    pub const DGF_DATA_SECTION_MAGIC: [u8; 4] = [b'.', b'd', b'a', b't'];

    #[derive(Copy, Clone)]
    #[repr(u16)]
    pub enum DgfVersion {
        Version1 = 1,
    }

    #[repr(C)]
    pub struct DgfHeader {
        pub magic: [u8; 4],
        pub version: DgfVersion,
        pub reserved: u16,
        pub properties: DgfImageProperties,
        pub compression: CompressionInfo,
        pub data_section_magic: [u8; 4],
        pub data_offset: u32,
        pub data_length: u32,
        pub data: [u8; 0],
    }

    #[derive(Copy, Clone)]
    pub struct DgfImageProperties {
        pub mode: ImageMode,
        pub width: u16,
        pub height: u16,
        pub reserved: [u8; 32],
    }

    #[derive(Copy, Clone)]
    #[repr(u16)]
    pub enum ImageMode {
        Grayscale1 = 0x00,
        Grayscale8 = 0x01,
        Rgb8 = 0x10,
    }

    pub mod compression {
        use crate::dgf::format::compression::image::ImageCompressionProps;
        use crate::dgf::format::compression::post::PostCompression;

        #[derive(Copy, Clone)]
        pub struct CompressionInfo {
            pub image: ImageCompressionProps,
            pub post: PostCompression,
        }

        pub mod image {
            #[derive(Copy, Clone)]
            pub struct ImageCompressionProps {
                pub mode: ImageCompressionMode,
                pub dct: DctProperties,
            }

            #[derive(Copy, Clone)]
            #[repr(u8)]
            pub enum ImageCompressionMode {
                None = 0,
                Dct = 1,
            }

            #[derive(Copy, Clone)]
            pub struct DctProperties {
                pub block_size: u8,
            }
        }

        pub mod post {
            #[derive(Copy, Clone)]
            pub struct PostCompression {
                pub mode: PostCompressionMode,
                pub reserved: [u8; 3],
            }

            #[derive(Copy, Clone)]
            #[repr(u8)]
            pub enum PostCompressionMode {
                None = 0,
                Rle = 1,
            }
        }
    }
}
