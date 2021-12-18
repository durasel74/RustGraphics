use sdl2;
use gl;

fn main() {
    // Initialization
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("RustGraphics", 800, 600)
        .opengl()
        .resizable()
        .build()
        .unwrap();
    let _gl_context = window.gl_create_context().unwrap();
    gl::load_with(|s| video_subsystem
        .gl_get_proc_address(s) as *const std::os::raw::c_void);

    unsafe { gl::ClearColor(0.2, 0.5, 0.6, 1.0); }

    let mut event_pump = sdl_context.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => break 'main,
                _ => (),
            }
        }

        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT); }
        
        window.gl_swap_window();
    }
}
