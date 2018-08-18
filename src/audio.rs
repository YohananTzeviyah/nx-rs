// Copyright © 2015-2018, Peter Atashian
//! Audio in NX files

/// Some audio, possibly a sound effect or music
#[derive(Clone, Copy)]
pub struct Audio<'a> {
    data:  &'a [u8],
    index: u32,
}

impl<'a> Audio<'a> {
    /// Creates an Audio from the supplied data
    #[inline]
    pub unsafe fn construct(data: &'a [u8], index: u32) -> Audio<'a> {
        Audio { data, index }
    }

    /// Returns the audio data, not including the wz audio header
    #[inline]
    pub fn data(&self) -> &[u8] {
        &self.data[82..]
    }

    /// Returns the wz audio header
    #[inline]
    pub fn header(&self) -> &[u8; 82] {
        assert!(self.data.len() >= 82);
        unsafe { &*(self.data.as_ptr() as *const [u8; 82]) }
    }

    /// Index of the audio within the offset table
    #[inline]
    pub fn index(&self) -> u32 {
        self.index
    }
}
