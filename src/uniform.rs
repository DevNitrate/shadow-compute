use std::ffi::CString;

pub trait Uniform {
    fn set(&self, loc: i32);
}

impl Uniform for i32 {
    fn set(&self, loc: i32) {
        unsafe {
            gl::Uniform1i(loc, *self);
        }
    }
}
impl Uniform for f32 {
    fn set(&self, loc: i32) {
        unsafe {
            gl::Uniform1f(loc, *self);
        }
    }
}

pub struct ShadowUniform<T: Uniform> {
    name: CString,
    value: T,
    uniform_loc: i32
}

impl<T: Uniform> ShadowUniform<T> {
    pub fn new(name: &str, value: T, program: u32) -> Self {
        let name: CString = CString::new(name).unwrap();

        unsafe {
            gl::UseProgram(program);
            let uniform_loc: i32 = gl::GetUniformLocation(program, name.as_ptr());
            value.set(uniform_loc);

            Self {
                name,
                value,
                uniform_loc
            }
        }
    }

    pub fn set(&mut self, value: T) {
        self.value = value;

        self.value.set(self.uniform_loc);
    }
}