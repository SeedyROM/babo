use std::{
    ffi::CString,
    path::{Path, PathBuf},
};

use nalgebra::{Matrix4, Orthographic3, Vector3};
use sdl2::{event::Event, keyboard::Keycode};

static VERTEX_SHADER_SRC: &str = "
    #version 330 core
    layout (location = 0) in vec3 aPos;

    uniform mat4 model;
    uniform mat4 view;
    uniform mat4 projection;

    void main() {
        gl_Position = projection * model * view * vec4(aPos, 1.0);
    }
";

static FRAGMENT_SHADER_SRC: &str = "
    #version 330 core
    out vec4 FragColor;

    uniform vec3 ourColor;

    void main() {
        FragColor = vec4(ourColor, 1.0);
    }
";

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ShaderType {
    Fragment,
    Vertex,
}

impl Into<u32> for ShaderType {
    fn into(self) -> u32 {
        match self {
            ShaderType::Fragment => gl::FRAGMENT_SHADER,
            ShaderType::Vertex => gl::VERTEX_SHADER,
        }
    }
}

pub struct Shader {
    id: u32,
    kind: ShaderType,
    source_location: PathBuf,
}

impl Shader {
    pub fn from_source(kind: ShaderType, source: &str) -> Result<Self, String> {
        let id = unsafe { gl::CreateShader(kind.into()) };
        let c_str = std::ffi::CString::new(source.as_bytes()).unwrap();
        unsafe {
            gl::ShaderSource(id, 1, &c_str.as_ptr(), std::ptr::null());
            gl::CompileShader(id);
        }

        let mut success = 0;
        unsafe {
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
        }

        if success == 0 {
            let mut len = 0;
            unsafe {
                gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = create_whitespace_cstring_with_len(len as usize);

            unsafe {
                gl::GetShaderInfoLog(id, len, std::ptr::null_mut(), error.as_ptr() as *mut _);
            }

            return Err(error.to_string_lossy().into_owned());
        }

        Ok(Shader {
            id,
            kind,
            source_location: PathBuf::new(),
        })
    }

    pub fn from_file(kind: ShaderType, path: &Path) -> Result<Self, String> {
        let source = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
        let shader = Shader::from_source(kind, &source)?;
        Ok(Shader {
            source_location: path.to_path_buf(),
            ..shader
        })
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn kind(&self) -> ShaderType {
        self.kind
    }

    pub fn source_location(&self) -> &Path {
        &self.source_location
    }
}

pub struct ShaderProgram {
    id: u32,
}

impl ShaderProgram {
    pub fn from_shaders(shaders: &[Shader]) -> Result<Self, String> {
        let id = unsafe { gl::CreateProgram() };

        for shader in shaders {
            unsafe {
                gl::AttachShader(id, shader.id());
            }
        }

        unsafe {
            gl::LinkProgram(id);
        }

        let mut success = 0;
        unsafe {
            gl::GetProgramiv(id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            let mut len = 0;
            unsafe {
                gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = create_whitespace_cstring_with_len(len as usize);

            unsafe {
                gl::GetProgramInfoLog(id, len, std::ptr::null_mut(), error.as_ptr() as *mut _);
            }

            return Err(error.to_string_lossy().into_owned());
        }

        for shader in shaders {
            unsafe {
                gl::DetachShader(id, shader.id());
            }
        }

        Ok(ShaderProgram { id })
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn set_used(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn set_uniform_1f(&self, name: &str, value: f32) {
        unsafe {
            gl::Uniform1f(self.get_uniform_location(name), value);
        }
    }

    pub fn set_uniform_1i(&self, name: &str, value: i32) {
        unsafe {
            gl::Uniform1i(self.get_uniform_location(name), value);
        }
    }

    pub fn set_uniform_3f(&self, name: &str, value: (f32, f32, f32)) {
        unsafe {
            gl::Uniform3f(self.get_uniform_location(name), value.0, value.1, value.2);
        }
    }

    pub fn set_uniform_4f(&self, name: &str, value: (f32, f32, f32, f32)) {
        unsafe {
            gl::Uniform4f(
                self.get_uniform_location(name),
                value.0,
                value.1,
                value.2,
                value.3,
            );
        }
    }

    pub fn set_uniform_mat4(&self, name: &str, value: &Matrix4<f32>) {
        unsafe {
            gl::UniformMatrix4fv(
                self.get_uniform_location(name),
                1,
                gl::FALSE,
                value.as_ptr(),
            );
        }
    }

    fn get_uniform_location(&self, name: &str) -> i32 {
        let name_cstr = std::ffi::CString::new(name).unwrap();
        let location = unsafe { gl::GetUniformLocation(self.id, name_cstr.as_ptr()) };

        if location == -1 {
            println!("Could not find uniform {}", name);
        }

        location
    }
}

#[inline(always)]
fn create_whitespace_cstring_with_len(len: usize) -> std::ffi::CString {
    // allocate buffer of correct size
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    // fill it with len spaces
    buffer.extend([b' '].iter().cycle().take(len));
    // convert buffer to CString
    unsafe { std::ffi::CString::from_vec_unchecked(buffer) }
}

fn main() {
    // Start SDL.
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    // Setup OpenGL attributes.
    video_subsystem
        .gl_attr()
        .set_context_profile(sdl2::video::GLProfile::Core);
    video_subsystem.gl_attr().set_context_version(3, 3);
    video_subsystem.gl_attr().set_context_flags().debug().set();

    // Create the window
    let window = video_subsystem
        .window("rust-sdl2 demo", 512, 512)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    // Create a new OpenGL context.
    let _gl_context = window.gl_create_context().unwrap();
    // Load the OpenGL function pointers.
    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const _);

    // Setup double buffering.
    video_subsystem.gl_set_swap_interval(1).unwrap();

    // Setup the viewport.
    unsafe {
        gl::Viewport(0, 0, 512, 512);
    }

    // Create a vao and vbo.
    let mut vao = 0;
    let mut vbo = 0;

    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        let vertices: [f32; 9] = [
            -1.0, -1.0, 0.0, // left
            1.0, -1.0, 0.0, // right
            0.0, 1.0, 0.0, // top
        ];

        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as isize,
            vertices.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );

        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
        gl::EnableVertexAttribArray(0);
    }

    // Create a vertex shader.
    let vertex_shader = Shader::from_source(ShaderType::Vertex, VERTEX_SHADER_SRC).unwrap();

    // Create a fragment shader.
    let fragment_shader = Shader::from_source(ShaderType::Fragment, FRAGMENT_SHADER_SRC).unwrap();

    // Create a shader program.
    let shader_program = ShaderProgram::from_shaders(&[vertex_shader, fragment_shader]).unwrap();

    // Set the color to clear the screen to.
    unsafe { gl::ClearColor(0.3, 0.3, 0.5, 1.0) };

    // Position of the triangle.
    let projection = Orthographic3::new(0.0, 512.0, 512.0, 0.0, -1.0, 1.0);
    let view = Matrix4::identity();

    let mut position = Vector3::new(0.0, 0.0, 1.0);
    let mut rotation = 0.0;

    // Handle events.
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        // Clear the screen.
        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT) };

        rotation += 0.01;
        position += Vector3::new(0.8, 0.8, 0.0);

        // Set the model matrix.
        let model = Matrix4::new_translation(&position);
        let model = model * Matrix4::new_rotation(&Vector3::z() * rotation);
        let model = model * Matrix4::new_nonuniform_scaling(&Vector3::new(128.0, 128.0, 1.0));

        // Use the shader program.
        shader_program.set_used();
        shader_program.set_uniform_3f("ourColor", (0.5, 1.0, 0.25));
        shader_program.set_uniform_mat4("projection", &projection.into_inner());
        shader_program.set_uniform_mat4("model", &model);
        shader_program.set_uniform_mat4("view", &view);

        // Bind the vao.
        unsafe { gl::BindVertexArray(vao) };
        // Draw the triangle.
        unsafe { gl::DrawArrays(gl::TRIANGLES, 0, 3) };

        // Swap the buffers.
        window.gl_swap_window();
    }
}
