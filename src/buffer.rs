use std::{ffi::c_void, ptr::null};

pub struct ShadowBuffer {
    idx: u32,
    ssbo: u32
}

impl ShadowBuffer {
    pub fn new(idx: u32, size: usize, usage: BufferUsage) -> Self {
        let mut ssbo: u32 = 0;

        unsafe {
            gl::GenBuffers(1, &mut ssbo);
            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, ssbo);
            gl::BufferData(ssbo, size as isize, null(), usage as u32);

            gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, idx, ssbo);
            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, 0);
        }

        Self {
            idx,
            ssbo
        }
    }

    pub fn update<T: Copy>(&self, data: &Vec<T>, offset: isize) {
        let size: isize = (size_of::<T>() * data.len()) as isize;

        unsafe {
            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, self.idx);
            gl::BufferSubData(gl::SHADER_STORAGE_BUFFER, offset, size, data.as_ptr() as *const c_void);
            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, 0);
        }
    }

    pub fn get_idx(&self) -> u32 {
        self.idx
    }
}

#[repr(u32)]
pub enum BufferUsage {
    GlStreamDraw = gl::STREAM_DRAW,
    GlStreamRead = gl::STREAM_READ,
    GlStreamCopy = gl::STREAM_COPY,
    GlStaticDraw = gl::STATIC_DRAW,
    GlStaticRead = gl::STATIC_READ,
    GlStaticCopy = gl::STATIC_COPY,
    GlDynamicDraw = gl::DYNAMIC_DRAW,
    GlDynamicRead = gl::DYNAMIC_READ,
    GlDynamicCopy = gl::DYNAMIC_COPY
}