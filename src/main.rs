//extern crate sdl2;
// pub mod main_old;
// pub mod render_gl;
//main_old::remain();

use sdl2;

macro_rules! wait {
    () => {
        std::io::stdin().read_line(&mut String::new()).unwrap();
    };
}

fn main() {
    let sdl_context = sdl2::init().unwrap();


    wait!();
}
