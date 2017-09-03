use std::fmt;
use std::mem;

/// Keeps track of the position when writing bytes to a buffer.
pub struct GlWriter<'a> {
    pub pos: usize,
    pub data: &'a mut [u8],
}

impl<'a> GlWriter<'a> {
    /// Create a new writer to buffer that starts at 0 position.
    pub fn new<'r>(data: &'r mut [u8]) -> GlWriter<'r> {
        GlWriter {
            pos: 0,
            data: data,
        }
    }

    /// Write a value to buffer and advance the position.
    pub fn write<T: fmt::Debug>(&mut self, val: T) {
        let type_size = mem::size_of::<T>();
        assert!(self.pos + type_size <= self.data.len());
        unsafe {
            *(self.data.as_mut_ptr().offset(self.pos as isize) as *mut T) = val;
        }
        self.pos += type_size;
    }
}
