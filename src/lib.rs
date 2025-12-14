use khronos_egl::{self as egl, BLUE_SIZE, CONTEXT_MAJOR_VERSION, CONTEXT_MINOR_VERSION, CONTEXT_OPENGL_CORE_PROFILE_BIT, CONTEXT_OPENGL_PROFILE_MASK, GREEN_SIZE, Instance, RED_SIZE, Static};

mod shader;
mod buffer;
mod uniform;

pub fn init_shadow() {
    let egl: Instance<Static> = Instance::new(egl::Static);
    let display = unsafe { egl.get_display(egl::DEFAULT_DISPLAY).expect("couldn't get display") };

    let attributes: [i32; 7] = [
        RED_SIZE, 8,
        GREEN_SIZE, 8,
        BLUE_SIZE, 8,
        egl::NONE
    ];

    let config = egl.choose_first_config(display, &attributes).unwrap().expect("coudln't choose first config");

    let context_attributes: [i32; 7] = [
        CONTEXT_MAJOR_VERSION, 4,
        CONTEXT_MINOR_VERSION, 6,
        CONTEXT_OPENGL_PROFILE_MASK, CONTEXT_OPENGL_CORE_PROFILE_BIT,
        egl::NONE
    ];

    egl.create_context(display, config, None, &context_attributes).expect("couldn't create egl context");

    gl::load_with(|s| egl.get_proc_address(s).expect("couldn't get proc address") as *const _);
}