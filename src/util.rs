use std::ops::Range;

#[derive(Debug)]
pub struct Chunk {
    pub x: u32,
    pub y: u32,
    pub size: u32,
}

impl Chunk {
    pub fn end_x(&self) -> u32 {
        self.x + self.size
    }

    pub fn end_y(&self) -> u32 {
        self.y + self.size
    }

    pub fn range_x(&self) -> Range<u32> {
        self.x..self.end_x()
    }

    pub fn range_y(&self) -> Range<u32> {
        self.y..self.end_y()
    }

    pub fn pixels(&self) -> u32 {
        self.size * self.size
    }
}

pub fn get_image_chunks(width: u32, height: u32, chunk_size: u32) -> Vec<Chunk> {
    let mut chunks = Vec::new();

    for y in (0..height).step_by(chunk_size as usize) {
        for x in (0..width).step_by(chunk_size as usize) {
            chunks.push(Chunk {
                x,
                y,
                size: chunk_size,
            });
        }
    }

    chunks
}
