use core::panic;
use std::{ffi::CString, fs, ptr::null};

use gl::types::{GLint, GLuint};

use crate::buffer::{BufferUsage, ShadowBuffer};

pub struct Shadow {
    program: GLuint,
    buffers: Vec<ShadowBuffer>,
}

impl Shadow {
    pub fn new(shader_path: &str) -> Self {
        let shader_source: String = fs::read_to_string(shader_path).expect("could not read shader source");
        let shader_source: CString = CString::new(shader_source).unwrap();

        let shader: u32;
        unsafe {
            shader = gl::CreateShader(gl::COMPUTE_SHADER);
            gl::ShaderSource(shader, 1, &shader_source.as_ptr(), null());
            gl::CompileShader(shader);

            if gl::GetError() != 0 {
                panic!("error compiling new shader");
            }
        }

        let program: u32 = unsafe { gl::CreateProgram() };
        unsafe {
            gl::AttachShader(program, shader);
            gl::LinkProgram(program);

            let mut success = gl::FALSE as GLint;
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
            if success == (gl::FALSE as GLint) {
                panic!("error linking shader program")
            }

            gl::DeleteShader(shader);
        };

        Self {
            program,
            buffers: Vec::new()
        }
    }

    pub fn new_buffer(&mut self, idx: u32, size: usize, usage: BufferUsage) {
        let buffer: ShadowBuffer = ShadowBuffer::new(idx, size, usage);
        self.buffers.push(buffer);
    }

    pub fn update_buffer<T: Copy>(&self, idx: u32, data: &Vec<T>, offset: isize) {
        self.buffers.iter().find(|b| b.get_idx() == idx).unwrap().update(data, offset);
    }

    pub fn get_program(&self) -> u32 {
        self.program
    }
}