use std::rc::Rc;
use gl;
use sdl2::{
    keyboard::Keycode,
    event::Event,
    video::GLProfile,
};
use sample_core::{
    shader::Shader,
    texture::Texture,
    mesh::Mesh,
    ecs::{ECS},
    component::components::*
};

use gltf;

mod scripts;
use scripts::plane::Plane;

const WIDTH: u32 = 1366 / 2;
const HEIGHT: u32 = 768 / 2;

// Mainly just testing code, eventually main should just read files
// TODO: Read files instead of hard code
fn main() {
    // ----- Init ----- //
    let context = sdl2::init().unwrap();
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
    gl::load_with(|name| video.gl_get_proc_address(name) as *const std::os::raw::c_void);
    // ----- !Init ----- //

    // ----- Textures ----- //
    //let texture = Rc::new(Texture::new("src/{{{IMAGE HERE}}}.png").unwrap());
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
    let mesh = add_model("src/boxes.gltf").expect("Model failed");
    /*let mesh = Rc::new(Mesh::new()
        .vertices(vertices)
        .indices(indices)
        .uv(uv)
        .build()
        .unwrap());*/
    // ----- !Mesh ----- //
    
    // ----- ECS ----- //
    let mut ecs = ECS::new();
    let entity = ecs.new_entity("Plane");
    entity.add_component::<MeshComponent>(&|comp| {
        comp.add_mstm(mesh.clone(), shader.clone(), None, None);
    });
    entity.add_component::<ScriptComponent<Plane>>(&|_| {} );
    // ----- !ECS ----- //

    unsafe {
        gl::Viewport(0, 0, WIDTH as i32, HEIGHT as i32);
        gl::ClearColor(0.3, 0.2, 0.3, 1.0);
    }

    let mut events = context.event_pump().unwrap();
    let _timer = context.timer().unwrap();

    'game_loop: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'game_loop;
                },
                _ => {}
            }
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        ecs.update();
        window.gl_swap_window();
    }
}

pub fn add_model<P: AsRef<std::path::Path>>(path: P) -> Result<Rc<Mesh>, String> {
    let (document, buffers, images) = match gltf::import(path) {
        Ok(t) => t,
        Err(e) => return Err(format!("{}", e)) 
    };

    let m = document.meshes().next().unwrap();
    let p = m.primitives().next().unwrap();

    let pos_accessor = p.get(&gltf::Semantic::Positions).unwrap();
    let pos_view = pos_accessor.view().unwrap();
    let pos_offset = pos_view.offset();
    let pos_length = pos_view.length();
    let pos_buffer = {
        let index = pos_view.buffer().index();
        let buffer = &buffers[index];
        &buffer[pos_offset..pos_offset + pos_length]
    };

    let ind_accessor = p.indices().unwrap();
    let ind_view = ind_accessor.view().unwrap();
    let ind_offset = ind_view.offset();
    let ind_length = ind_view.length();
    let ind_buffer = {
        let index = ind_view.buffer().index();
        let buffer = &buffers[index];
        &buffer[ind_offset..ind_offset + ind_length]
    };

    println!("Index count from buffer reference: {}", ind_buffer.len());
    print!("Vertices ");
    let buf_pointer = pos_buffer.as_ptr() as *const f32;
    for i in (0..10).step_by(3) {
        unsafe { print!("[ {} {} {} ] ", *(buf_pointer.offset(i)), *(buf_pointer.offset(i + 1)), *(buf_pointer.offset(i + 1))); }
    }
    println!("");

    let vertices = pos_buffer.to_vec();
    let indices = ind_buffer.to_vec();

    let mesh = Rc::new(Mesh::new()
        .vertices(vertices)
        .indices(indices)
        .uv(Vec::new())
        .build()
        .unwrap()
    );

    return Ok(mesh);
}