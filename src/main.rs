use std::rc::Rc;
use gl;
use sdl2::{
    keyboard::Keycode,
    event::Event,
    video::GLProfile,
    image
};
use gl_wrap_core::{
    shader::Shader,
    texture::Texture,
    mesh::Mesh,
    ecs::{ECS},
    component::components::*
};

mod plane;
use crate::plane::Plane;

const WIDTH: u32 = 1366 / 2;
const HEIGHT: u32 = 768 / 2;

fn main() {
    // ----- Init ----- //
    /*let context = sdl2::init().unwrap();
    let _img_context = image::init(image::InitFlag::JPG).unwrap();
    let video = context.video().unwrap();

    let gl_attr = video.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(4, 1);

    let window = video.window("SDL / OpenGL", WIDTH, HEIGHT)
        .opengl()
        .resizable()
        .position_centered()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    gl::load_with(|name| video.gl_get_proc_address(name) as *const std::os::raw::c_void);*/
    // ----- !Init ----- //

    // ----- Textures ----- //
    let texture = Rc::new(Texture::new("src/lonely-japanese-cherry.jpg").unwrap());
    // ----- !Textures ----- //
    
    // ----- Shader ----- //
    let shader = Rc::new(Shader::new()
        .vertex("shaders/vertex.glsl")
        .fragment("shaders/fragment.glsl")
        .perspective(WIDTH, HEIGHT)
        .build()
        .unwrap());
    // ----- !Shader ----- //

    // ------ Data ----- //
    let vertices: Vec<gl::types::GLfloat> = vec![
        0.5, 0.5, 0.0,
        0.5, -0.5, 0.0,
        -0.5, -0.5, 0.0,
        -0.5, 0.5, 0.0,
    ];

    let indices: Vec<gl::types::GLuint> = vec![
        0, 1, 3,
        1, 2, 3
    ];

    let uv: Vec<gl::types::GLfloat> = vec![
        1.0, 0.0,
        1.0, 1.0,
        0.0, 1.0,
        0.0, 0.0
    ];
    // ------ !Data ----- //

    // ----- Mesh ----- //
    let mesh = Rc::new(Mesh::new()
        .vertices(vertices)
        .indices(indices)
        .uv(uv)
        .build()
        .unwrap());
    // ----- !Mesh ----- //
    
    // ----- ECS ----- //
    let mut ecs = ECS::new();
    let entity = ecs.new_entity("Plane");
    entity.add_component::<MeshComponent>(&|comp| {
        comp.add_mstm(mesh.clone(), shader.clone(), Some(texture.clone()), None);
    });
    entity.add_component::<ScriptComponent<Plane>>(&|_| {} );
    // ----- !ECS ----- //

    unsafe {
        gl::Viewport(0, 0, WIDTH as i32, HEIGHT as i32);
        gl::ClearColor(0.3, 0.2, 0.3, 1.0);
    }

    //let mut events = context.event_pump().unwrap();
    //let _timer = context.timer().unwrap();

    for _ in 0..100 {
    //'game_loop: loop {
        /*for event in events.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'game_loop;
                },
                _ => {}
            }
        }*/

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        ecs.update();
        //window.gl_swap_window();
    }
}
