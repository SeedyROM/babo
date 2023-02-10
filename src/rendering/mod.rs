pub mod camera;
pub mod window;

pub use camera::*;
pub use window::*;

// OpenGL renderering.
#[cfg(feature = "opengl")]
pub mod opengl;
#[cfg(feature = "opengl")]
pub use opengl::*;

// SDL2 windowing / input layer.
#[cfg(feature = "sdl2")]
pub mod sdl2;
#[cfg(feature = "sdl2")]
pub use crate::rendering::sdl2::*;
