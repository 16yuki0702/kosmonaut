use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::{ContextBuilder, GlProfile, PossiblyCurrent, WindowedContext};

pub mod kgl;
pub mod program;
pub mod shader;

pub fn init_main_window_and_gl() -> (WindowedContext<PossiblyCurrent>, EventLoop<()>) {
    let el = EventLoop::new();
    let wb = WindowBuilder::new().with_title("Kosmonaut");
    let windowed_context = ContextBuilder::new()
        .with_gl_profile(GlProfile::Core)
        .build_windowed(wb, &el)
        .unwrap();
    let windowed_context = unsafe { windowed_context.make_current().unwrap() };
    // TODO: Do we need to set viewport size here?
    let gl_context = windowed_context.context();
    gl::load_with(|ptr| gl_context.get_proc_address(ptr) as *const _);
    (windowed_context, el)
}

pub fn run_event_loop(
    windowed_context: WindowedContext<PossiblyCurrent>,
    event_loop: EventLoop<()>,
) {
    #[rustfmt::skip]
    static VERTEX_DATA: [f32; 9] = [
        -0.5, -0.5, 0.0,
        0.5, -0.5, 0.0,
        0.0,  0.5, 0.0
    ];

    event_loop.run(move |event, _, control_flow| {
        //        println!("{:?}", event);
        *control_flow = ControlFlow::Wait;
        match event {
            Event::LoopDestroyed => {}
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::Resized(logical_size) => {
                    let dpi_factor = windowed_context.window().hidpi_factor();
                    windowed_context.resize(logical_size.to_physical(dpi_factor));
                }
                WindowEvent::RedrawRequested => {
                    // TODO: Do we need to set viewport size here?
                    unsafe {
                        gl::ClearColor(0.1, 0.9, 0.3, 1.0);
                        gl::Clear(gl::COLOR_BUFFER_BIT);
                        gl::DrawArrays(gl::TRIANGLES, 0, 3);
                    }
                    windowed_context.swap_buffers().unwrap();
                }
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            _ => (),
        }
    });
}
