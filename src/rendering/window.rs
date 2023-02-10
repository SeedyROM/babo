use sdl2::event::Event;
use snafu::Snafu;

use crate::gl_unchecked;

#[derive(Debug, Snafu)]
pub enum WindowError {
    #[snafu(display("Renderer error: {}", message))]
    RendererError { message: String },
}

impl From<String> for WindowError {
    fn from(error: String) -> Self {
        WindowError::RendererError { message: error }
    }
}

pub trait WindowTrait<'a> {
    type Event;

    fn size(&self) -> (u32, u32);
    fn set_title(&mut self, title: &str);
    fn running(&self) -> bool;
    fn stop(&mut self);

    fn events(&'a mut self) -> Vec<Self::Event>;
    fn clear(&mut self, r: f32, g: f32, b: f32);
    fn present(&mut self);
}

pub struct Window {
    pub width: u32,
    pub height: u32,
    pub title: String,
    window: sdl2::video::Window,
    _sdl_context: sdl2::Sdl,
    _gl_context: sdl2::video::GLContext,
    event_pump: sdl2::EventPump,
    should_close: bool,
}

impl Window {
    pub fn new(width: u32, height: u32, title: &str) -> Result<Self, WindowError> {
        let _sdl_context = sdl2::init()?;

        let video_subsystem = _sdl_context.video()?;

        video_subsystem
            .gl_attr()
            .set_context_profile(sdl2::video::GLProfile::Core);
        video_subsystem.gl_attr().set_context_version(3, 3);
        video_subsystem.gl_attr().set_context_flags().debug().set();

        let window = video_subsystem
            .window(title, width, height)
            .position_centered()
            .opengl()
            .build()
            .map_err(|err| err.to_string())?;

        let _gl_context = window.gl_create_context()?;
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const _);

        video_subsystem.gl_set_swap_interval(1)?;

        let event_pump = _sdl_context.event_pump()?;

        Ok(Window {
            width,
            height,
            title: title.to_string(),
            window,
            _sdl_context,
            _gl_context,
            event_pump,
            should_close: false,
        })
    }
}

impl<'a> WindowTrait<'a> for Window {
    type Event = Event;

    fn size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    fn set_title(&mut self, title: &str) {
        self.window.set_title(title).unwrap();
    }

    fn running(&self) -> bool {
        !self.should_close
    }

    fn stop(&mut self) {
        self.should_close = true;
    }

    fn events(&'a mut self) -> Vec<Self::Event> {
        self.event_pump.poll_iter().collect()
    }

    fn clear(&mut self, r: f32, g: f32, b: f32) {
        gl_unchecked!(ClearColor, r, g, b, 1.0);
        gl_unchecked!(Clear, gl::COLOR_BUFFER_BIT);
        gl_unchecked!(Clear, gl::DEPTH_BUFFER_BIT);
    }

    fn present(&mut self) {
        self.window.gl_swap_window();
    }
}
