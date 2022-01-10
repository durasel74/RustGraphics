use sdl2;
use sdl2::event::WindowEvent;
use gl;

/// Представляет окно для работы с OpenGL.
pub struct GLWindow {
    window: sdl2::video::Window,
    event_pump: sdl2::EventPump,
    gl_context: sdl2::video::GLContext,
    pub arrowv_event: i32,
    pub arrowh_event: i32,
    pub draw_mode: u32,
    pub circles_mode: u32,
}
impl GLWindow {
    /// Создает новое окно с параметрами по умолчанию.
    pub fn new() -> Self {
        Self::from_parameters("Window", 800, 600)
    }

    /// Создает окно с указанными размерами и заголовком.
    pub fn from_parameters(title: &str, width: u32, height: u32) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(3, 3);

        let window = video_subsystem
            .window(title, width, height)
            .opengl()
            .resizable()
            .build()
            .unwrap();

        let event_pump = sdl_context.event_pump().unwrap();
        let gl_context = window.gl_create_context().unwrap();
        gl::load_with(|s| video_subsystem
            .gl_get_proc_address(s) as *const std::os::raw::c_void);

        GLWindow { window, event_pump, gl_context, 
            arrowv_event: 0, arrowh_event: 0, draw_mode: 0, circles_mode: 0 }
    }

    /// Обновляет окно.
    pub fn update(&self) {
        self.window.gl_swap_window();
    }

    /// Реагирует на события окна.
    pub fn event_check(&mut self) -> bool {
        for event in self.event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => return false,
                sdl2::event::Event::KeyUp { 
                    keycode: Some(sdl2::keyboard::Keycode::Escape), ..} => 
                    return false,
                sdl2::event::Event::Window { 
                    win_event: WindowEvent::Resized(width, height), ..} => 
                        unsafe { gl::Viewport(0, 0, width, height); },

                sdl2::event::Event::KeyUp { 
                    keycode: Some(sdl2::keyboard::Keycode::Up), ..} => { 
                        self.arrowv_event = 1 },
                sdl2::event::Event::KeyDown { 
                    keycode: Some(sdl2::keyboard::Keycode::Up), repeat: true, ..} => { 
                        self.arrowv_event = 1 },
                sdl2::event::Event::KeyUp { 
                    keycode: Some(sdl2::keyboard::Keycode::Down), ..} => { 
                        self.arrowv_event = -1 },
                sdl2::event::Event::KeyDown { 
                    keycode: Some(sdl2::keyboard::Keycode::Down), repeat: true, ..} => { 
                        self.arrowv_event = -1 },

                sdl2::event::Event::KeyUp { 
                    keycode: Some(sdl2::keyboard::Keycode::Left), ..} => { 
                        self.arrowh_event = -1 },
                sdl2::event::Event::KeyDown { 
                    keycode: Some(sdl2::keyboard::Keycode::Left), repeat: true, ..} => { 
                        self.arrowh_event = -1 },
                sdl2::event::Event::KeyUp { 
                    keycode: Some(sdl2::keyboard::Keycode::Right), ..} => { 
                        self.arrowh_event = 1 },
                sdl2::event::Event::KeyDown { 
                    keycode: Some(sdl2::keyboard::Keycode::Right), repeat: true, ..} => { 
                        self.arrowh_event = 1 },
                sdl2::event::Event::KeyUp { 
                    keycode: Some(sdl2::keyboard::Keycode::M), ..} => { 
                        self.circles_mode = (self.circles_mode == 0) as u32 },
                
                sdl2::event::Event::KeyUp {
                    keycode: Some(sdl2::keyboard::Keycode::Tab), ..} => { 
                        self.draw_mode = (self.draw_mode + 1) % 3 },

                _ => (),
            }
        }
        return true;
    }
}
