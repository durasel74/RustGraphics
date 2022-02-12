use sdl2;
use sdl2::event::{ WindowEvent, Event };
use gl;

/// Представляет окно для работы с OpenGL.
pub struct GLWindow {
    sdl_context: sdl2::Sdl,
    pub window: sdl2::video::Window,
    event_pump: sdl2::EventPump,
    gl_context: sdl2::video::GLContext,
    fullscreen_mode: bool,
    pub draw_mode: u32,
    pub arrow_v: i8,
    pub arrow_h: i8,
    pub delta_x: f32,
    pub delta_y: f32,
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

        let mut window = video_subsystem
            .window(title, width, height)
            .opengl()
            .resizable()
            .build()
            .unwrap();
        window.set_minimum_size(600, 200).unwrap();

        let event_pump = sdl_context.event_pump().unwrap();
        let gl_context = window.gl_create_context().unwrap();
        gl::load_with(|s| video_subsystem
            .gl_get_proc_address(s) as *const std::os::raw::c_void);
        //sdl_context.mouse().show_cursor(true);

        GLWindow { sdl_context, window, event_pump, gl_context,
            fullscreen_mode: false, draw_mode: 0, arrow_h: 0, arrow_v: 0,
            delta_x: 400.0, delta_y: 350.0 }
    }

    pub fn sdl_window(&self) -> &sdl2::video::Window { &self.window }

    /// Обновляет окно.
    pub fn update(&self) {
        self.window.gl_swap_window();
    }

    /// Реагирует на события окна.
    pub fn event_check(&mut self) -> bool {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit {..} => return false,
                Event::Window { 
                    win_event: WindowEvent::Resized(width, height), ..} => 
                        unsafe { gl::Viewport(0, 0, width, height); },
                Event::KeyUp { 
                    keycode: Some(sdl2::keyboard::Keycode::Escape), ..} => 
                    return false,
                Event::KeyUp {
                    keycode: Some(sdl2::keyboard::Keycode::Tab), ..} =>
                        self.draw_mode = (self.draw_mode + 1) % 3,
                Event::KeyUp {
                    keycode: Some(sdl2::keyboard::Keycode::Return), ..} => {
                        if !self.fullscreen_mode {
                            self.window.set_fullscreen(sdl2::video::FullscreenType::Desktop).unwrap();
                            self.fullscreen_mode = true;
                        }
                        else {
                            self.window.set_fullscreen(sdl2::video::FullscreenType::Off).unwrap();
                            unsafe { gl::Viewport(0, 0, self.window.size().0 as i32, self.window.size().1 as i32); }
                            self.fullscreen_mode = false;
                        }
                    },
                Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::W), ..} =>
                        self.arrow_v += 1,
                Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::S), ..} =>
                        self.arrow_v -= 1,
                Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::D), ..} =>
                        self.arrow_h += 1,
                Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::A), ..} =>
                        self.arrow_h -= 1,
                Event::MouseMotion {x, y, .. } => 
                {
                    let win_width = self.window.size().0 as f32 / 2.0;
                    let win_height = self.window.size().1 as f32 / 2.0;
                    self.delta_x = x as f32 - win_width;
                    self.delta_y = y as f32 - win_height;
                    // self.sdl_context.mouse().warp_mouse_in_window(&self.window, 
                    //     win_width as i32, win_height as i32);
                },
                _ => (),
            }
        }
        return true;
    }
}
