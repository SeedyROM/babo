use nalgebra::{Matrix4, Vector2, Vector3};

use super::{Shader, ShaderProgram, ShaderType, Texture};
use crate::gl;

static QUAD_VERTICES: [f32; 24] = [
    0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0,
    1.0, 1.0, 0.0, 1.0, 0.0,
];

pub struct SpriteRenderer {
    shader: ShaderProgram,
    quad_vao: u32,
    _quad_vbo: u32,
}

impl SpriteRenderer {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Create the shader program for the sprite renderer
        let shader = ShaderProgram::from_shaders(&[
            Shader::from_source(
                ShaderType::Vertex,
                include_str!("../../assets/shaders/sprite.vert"),
            )
            .unwrap(),
            Shader::from_source(
                ShaderType::Fragment,
                include_str!("../../assets/shaders/sprite.frag"),
            )
            .unwrap(),
        ])
        .unwrap();

        // Setup the quad VAO and VBO
        let mut quad_vao = 0;
        let mut quad_vbo = 0;

        // Bind the VAO
        gl!(GenVertexArrays, 1, &mut quad_vao)?;
        gl!(BindVertexArray, quad_vao)?;

        // Bind the VBO
        gl!(GenBuffers, 1, &mut quad_vbo)?;
        gl!(BindBuffer, gl::ARRAY_BUFFER, quad_vbo)?;

        // Buffer the quad data.
        gl!(
            BufferData,
            gl::ARRAY_BUFFER,
            (QUAD_VERTICES.len() * std::mem::size_of::<f32>()) as isize,
            QUAD_VERTICES.as_ptr() as *const _,
            gl::STATIC_DRAW,
        )?;

        // Setup the vertex attributes
        gl!(
            VertexAttribPointer,
            0,
            2,
            gl::FLOAT,
            gl::FALSE,
            4 * std::mem::size_of::<f32>() as i32,
            std::ptr::null(),
        )?;
        gl!(EnableVertexAttribArray, 0)?;
        gl!(
            VertexAttribPointer,
            1,
            2,
            gl::FLOAT,
            gl::FALSE,
            4 * std::mem::size_of::<f32>() as i32,
            (2 * std::mem::size_of::<f32>()) as *const _,
        )?;
        gl!(EnableVertexAttribArray, 1)?;

        Ok(Self {
            shader,
            quad_vao,
            _quad_vbo: quad_vbo,
        })
    }

    pub fn draw(
        &self,
        texture: &Texture,
        projection: &Matrix4<f32>,
        view: &Matrix4<f32>,
        position: &Vector3<f32>,
        size: &Vector2<f32>,
        rotation: f32,
        color: &Vector3<f32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Enable blending.
        gl!(Enable, gl::BLEND)?;
        gl!(BlendFunc, gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA)?;

        // Use the shader program
        self.shader.use_program();

        // Calculate the transform matrix.
        let model = Matrix4::new_translation(position)
            * Matrix4::new_translation(&Vector3::new(0.5 * size.x, 0.5 * size.y, 0.0))
            * Matrix4::new_rotation(&Vector3::z() * rotation)
            * Matrix4::new_translation(&Vector3::new(-0.5 * size.x, -0.5 * size.y, 0.0))
            * Matrix4::new_nonuniform_scaling(&Vector3::new(size.x, size.y, 1.0));

        let transform = projection * view * model;

        // Bind the texture and vertex array
        gl!(BindVertexArray, self.quad_vao)?;
        gl!(ActiveTexture, gl::TEXTURE0)?;
        gl!(BindTexture, gl::TEXTURE_2D, texture.id())?;

        // Set the uniforms
        self.shader.set_uniform_mat4("transform", &transform)?;
        self.shader
            .set_uniform_3f("spriteColor", (color.x, color.y, color.z))?;

        // Draw the quad
        gl!(DrawArrays, gl::TRIANGLES, 0, 6)?;
        gl!(BindVertexArray, 0)?;

        // Disable blending.
        gl!(Disable, gl::BLEND)?;

        Ok(())
    }
}
