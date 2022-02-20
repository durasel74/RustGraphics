// use std::ffi;

// use glutin;
// use glutin::window;
// use glutin::event;
// use glutin::event_loop;
// use glutin::dpi;

// use gl;

// pub struct Window {
//     windowed_context: glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>,
// }

// impl Window {
//     pub fn new(window_builder: window::WindowBuilder, 
//     event_loop: &event_loop::EventLoop<()>) -> Self {
//         let windowed_context = glutin::ContextBuilder::new()
//             .build_windowed(window_builder, event_loop)
//             .unwrap();
//         let windowed_context = unsafe { windowed_context.make_current().unwrap() };
//         Self::gl_load(windowed_context.context());
//         Window { windowed_context }
//     }

//     pub fn windowed_context(&self) -> &glutin::ContextWrapper<glutin::PossiblyCurrent, 
//     glutin::window::Window> {
//         &self.windowed_context
//     }

//     pub fn inner_size(&self) -> (u32, u32) {
//         let physical_window = self.windowed_context().window();
//         (physical_window.inner_size().width, physical_window.inner_size().height)
//     }

//     pub fn run<F: FnMut(&Window)>(&self, event_loop: event_loop::EventLoop<()>, main_loop: F){

//         event_loop.run(move |event, _, control_flow| {
//             *control_flow = event_loop::ControlFlow::Poll;

//             match event {
//                 event::Event::LoopDestroyed => return,
//                 event::Event::WindowEvent { event, .. } =>
//                     Self::window_event_handler(event, control_flow),
//                 event::Event::MainEventsCleared => {
//                     main_loop(self);
//                 }
//                 _ => (),
//             }
//         });
//     }

//     pub fn update(&self) {
//         self.windowed_context.swap_buffers().unwrap();
//     }

//     fn window_event_handler(event: event::WindowEvent, 
//     control_flow: &mut event_loop::ControlFlow) {
//         match event {
//             event::WindowEvent::CloseRequested =>
//                 *control_flow = event_loop::ControlFlow::Exit,
//             event::WindowEvent::Resized(physical_size) => unsafe {
//                 gl::Viewport(0, 0, physical_size.width as i32, physical_size.height as i32); },
//             _ => ()
//         }
//     }

//     fn gl_load(gl_context: &glutin::Context<glutin::PossiblyCurrent>) {
//         gl::load_with(|ptr| gl_context.get_proc_address(ptr) as *const _);

//         let version = unsafe {
//             let data = ffi::CStr::from_ptr(gl::GetString(gl::VERSION) as *const _)
//                 .to_bytes().to_vec();
//             String::from_utf8(data).unwrap()
//         };
//         println!("OpenGL version {}", version);
//     }
// }
