extern crate glutin;

use glutin::event::{ElementState, Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::{Api, ContextBuilder, GlProfile, GlRequest};

mod display;
mod gfx;
mod utils;

fn main() {
    let el = EventLoop::new();
    let wb = WindowBuilder::new().with_title("Warnengine");

    let windowed_context = ContextBuilder::new()
        .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
        .with_gl_profile(GlProfile::Core)
        .build_windowed(wb, &el)
        .unwrap();

    let windowed_context = unsafe { windowed_context.make_current().unwrap() };

    gl::load_with(|s| windowed_context.get_proc_address(s) as *const _);

    gfx::info();

    unsafe {
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }

    let basic = gfx::create_program("./assets/basic.vs.glsl", "./assets/basic.fs.glsl");
    let triangle = gfx::gen_vertex_array(9);
    let vertices = vec![
        0.0, -1.0, 0.0, // lower-left corner
        0.0, 0.0, 0.0, // lower-right corner
        1.0, 0.0, 0.0, // top-left corner
        1.0, -1.0, 0.0, 1.0, 0.0, 0.0, 0.0, -1.0, 0.0,
    ];
    let uvs = vec![
        0.0, 0.0, // lower-left corner
        0.0, -1.0, // lower-right corner
        1.0, -1.0, // top-left corner
        1.0, 0.0, 1.0, -1.0, 0.0, 0.0,
    ];
    gfx::gen_buffer(&triangle, vertices, 3, 0);
    gfx::gen_buffer(&triangle, uvs, 2, 1);

    let texture = gfx::tex_image_2d("./assets/oui.png");

    el.run(move |event, _, control_flow| {
        // println!("{:?}", event);
        *control_flow = ControlFlow::Poll;

        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(physical_size) => windowed_context.resize(physical_size),
                WindowEvent::MouseInput {
                    state: ElementState::Released,
                    ..
                } => {
                    println!("ici");
                }
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            _ => (),
        }
        gfx::clear();
        gfx::use_program(basic);
        gfx::bind_texture(texture, gfx::TEXTURE0);
        gfx::draw_arrays(&triangle);
        windowed_context.swap_buffers().unwrap();
    });
}
