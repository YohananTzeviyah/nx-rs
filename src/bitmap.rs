// Copyright © 2015-2018, Peter Atashian
//! Bitmaps in NX files
use lz4::{decompress};

/// Represents a bitmap
#[derive(Clone, Copy)]
pub struct Bitmap<'a> {
    width: u16,
    height: u16,
    data: &'a [u8],
}
impl<'a> Bitmap<'a> {
    /// The width in pixels
    #[inline]
    pub fn width(&self) -> u16 {
        self.width
    }
    /// The height in pixels
    #[inline]
    pub fn height(&self) -> u16 {
        self.height
    }
    /// The length of the decompressed data in bytes
    #[inline]
    pub fn len(&self) -> u32 {
        self.width as u32 * self.height as u32 * 4
    }
    /// Creates a `Bitmap` from the supplied data
    #[inline]
    pub unsafe fn construct(data: &'a [u8], width: u16, height: u16) -> Bitmap<'a> {
        Bitmap { width: width, height: height, data: data }
    }
    /// Decompresses the bitmap data into the provided buffer
    #[inline]
    pub fn data(&self, out: &mut [u8]) {
        assert_eq!(out.len(), self.len() as usize);
        let len = decompress(self.data, out);
        assert_eq!(len, Ok(self.len() as usize));
    }
    /// The raw (LZ4-compressed) bitmap data
    #[inline]
    pub fn raw_data(&self) -> &[u8] {
        self.data
    }
}
