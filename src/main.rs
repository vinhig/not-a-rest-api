extern crate glutin;

use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::{ContextBuilder, GlRequest, Api, GlProfile};

mod gfx;
mod display;
mod utils;

fn main() {
    let el = EventLoop::new();
    let wb = WindowBuilder::new().with_title("Warnengine");

    let windowed_context = ContextBuilder::new()
        .with_gl(GlRequest::Specific(Api::OpenGl, (4, 3)))
        .with_gl_profile(GlProfile::Core)
        .build_windowed(wb, &el)
        .unwrap();

    let windowed_context = unsafe { windowed_context.make_current().unwrap() };

    gl::load_with(|s| windowed_context.get_proc_address(s) as *const _);

    let basic = gfx::create_program("./assets/basic.vs.glsl", "./assets/basic.fs.glsl");
    let triangle = gfx::gen_vertex_array(9);
    let vertices = vec![
        -0.5, -0.5, 0.0,
        0.5, -0.5, 0.0,
        0.0, 0.5, 0.0
    ];
    gfx::gen_buffer(&triangle, vertices, 3, 0);

    gfx::info();

    el.run(move |event, _, control_flow| {
        // println!("{:?}", event);
        *control_flow = ControlFlow::Wait;

        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(physical_size) => windowed_context.resize(physical_size),
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            Event::RedrawRequested(_) => {
                gfx::clear();
                gfx::use_program(basic);
                gfx::draw_arrays(&triangle);
                windowed_context.swap_buffers().unwrap();
            }
            _ => (),
        }
    });
}