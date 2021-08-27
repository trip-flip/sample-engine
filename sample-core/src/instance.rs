use sdl2::event::{Event};
use sdl2::keyboard::Keycode;
use sdl2::video::GLProfile;
use sdl2::image;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

pub struct GameInstance {
    pub sdl_context: sdl2::Sdl,
    pub img_context: sdl2::image::Sdl2ImageContext,
    pub vid_context: sdl2::VideoSubsystem,
    pub gl_context: sdl2::video::GLContext,
    pub window: sdl2::video::Window,
}

impl GameInstance {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let img_context = image::init(image::InitFlag::JPG).unwrap();
        let vid_context = sdl_context.video().unwrap();

        let gl_attr = vid_context.gl_attr();
        gl_attr.set_context_profile(GLProfile::Core);
        gl_attr.set_context_version(4, 1);

        let window = vid_context.window("SDL / OpenGL", WIDTH, HEIGHT)
            .opengl()
            .resizable()
            .position_centered()
            .build()
            .unwrap();

        let gl_context = window.gl_create_context().unwrap();
        gl::load_with(|name| vid_context.gl_get_proc_address(name) as *const std::os::raw::c_void);

        GameInstance {
            sdl_context: sdl_context,
            img_context: img_context,
            vid_context: vid_context,
            gl_context: gl_context,
            window: window,
        }
    }
}
