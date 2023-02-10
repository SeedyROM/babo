#![allow(unused)]

use std::{
    ffi::CString,
    path::{Path, PathBuf},
};

use nalgebra::Matrix4;
use snafu::Snafu;

use crate::{
    gl,
    utils::{create_whitespace_cstring_with_len, GlError},
};

#[derive(Debug, Snafu)]
pub enum ShaderError {
    #[snafu(display("Graphics error (code {}): {}", error.code, error.message))]
    GlError { error: GlError },
    #[snafu(display("Uniform not found: {}", name))]
    UniformNotFound { name: String },
}

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

pub struct ShaderSource<'a>(&'a str);

impl<'a> ShaderSource<'a> {
    pub fn into_inner(self) -> CString {
        self.into()
    }
}

impl<'a> From<&'a str> for ShaderSource<'a> {
    fn from(source: &'a str) -> Self {
        Self(source)
    }
}

impl<'a> Into<CString> for ShaderSource<'a> {
    fn into(self) -> CString {
        CString::new(self.0).unwrap()
    }
}

pub struct Shader {
    id: u32,
    kind: ShaderType,
    source_location: PathBuf,
}

impl Shader {
    pub fn from_source(kind: ShaderType, source: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let id = gl!(CreateShader, kind.into())?;

        let shader_source = ShaderSource::from(source);
        gl!(
            ShaderSource,
            id,
            1,
            &shader_source.into_inner().as_ptr(),
            std::ptr::null()
        );
        gl!(CompileShader, id);

        check_shader_error(id, gl::COMPILE_STATUS, false)?;

        Ok(Shader {
            id,
            kind,
            source_location: PathBuf::new(),
        })
    }

    pub fn from_file(
        kind: ShaderType,
        path: impl Into<PathBuf>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let path = path.into();
        let source = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
        let shader = Shader::from_source(kind, &source)?;
        Ok(Shader {
            source_location: path,
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
    pub fn from_shaders(shaders: &[Shader]) -> Result<Self, Box<dyn std::error::Error>> {
        let id = gl!(CreateProgram)?;

        for shader in shaders {
            gl!(AttachShader, id, shader.id())?;
        }

        gl!(LinkProgram, id);

        check_shader_error(id, gl::LINK_STATUS, true)?;

        for shader in shaders {
            gl!(DetachShader, id, shader.id())?;
        }

        Ok(ShaderProgram { id })
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn use_program(&self) {
        gl!(UseProgram, self.id);
    }

    pub fn set_uniform_1f(&self, name: &str, value: f32) -> Result<(), ShaderError> {
        gl!(Uniform1f, self.get_uniform_location(name)?, value)
            .map_err(|e| ShaderError::GlError { error: e })
    }

    pub fn set_uniform_1i(&self, name: &str, value: i32) -> Result<(), ShaderError> {
        gl!(Uniform1i, self.get_uniform_location(name)?, value)
            .map_err(|e| ShaderError::GlError { error: e })
    }

    pub fn set_uniform_3f(&self, name: &str, value: (f32, f32, f32)) -> Result<(), ShaderError> {
        gl!(
            Uniform3f,
            self.get_uniform_location(name)?,
            value.0,
            value.1,
            value.2
        )
        .map_err(|e| ShaderError::GlError { error: e })
    }

    pub fn set_uniform_4f(
        &self,
        name: &str,
        value: (f32, f32, f32, f32),
    ) -> Result<(), ShaderError> {
        gl!(
            Uniform4f,
            self.get_uniform_location(name)?,
            value.0,
            value.1,
            value.2,
            value.3
        )
        .map_err(|e| ShaderError::GlError { error: e })
    }

    pub fn set_uniform_mat4(&self, name: &str, value: &Matrix4<f32>) -> Result<(), ShaderError> {
        gl!(
            UniformMatrix4fv,
            self.get_uniform_location(name)?,
            1,
            gl::FALSE,
            value.as_ptr(),
        )
        .map_err(|e| ShaderError::GlError { error: e })
    }

    fn get_uniform_location(&self, name: &str) -> Result<i32, ShaderError> {
        let name_cstr = CString::new(name).unwrap();
        let location = unsafe { gl::GetUniformLocation(self.id, name_cstr.as_ptr()) };

        if location == -1 {
            return Err(ShaderError::UniformNotFound {
                name: name.to_string(),
            });
        }

        Ok(location)
    }
}

fn check_shader_error(shader: u32, flag: u32, is_program: bool) -> Result<(), String> {
    let mut success = 0;
    let mut len = 0;

    if is_program {
        unsafe {
            gl::GetProgramiv(shader, flag, &mut success);
        }
    } else {
        unsafe {
            gl::GetShaderiv(shader, flag, &mut success);
        }
    }

    if success == 0 {
        if is_program {
            unsafe {
                gl::GetProgramiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            }
        } else {
            unsafe {
                gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            }
        }

        let error = create_whitespace_cstring_with_len(len as usize);

        if is_program {
            unsafe {
                gl::GetProgramInfoLog(shader, len, std::ptr::null_mut(), error.as_ptr() as *mut _);
            }
        } else {
            unsafe {
                gl::GetShaderInfoLog(shader, len, std::ptr::null_mut(), error.as_ptr() as *mut _);
            }
        }

        return Err(error.to_string_lossy().into_owned());
    }

    Ok(())
}
