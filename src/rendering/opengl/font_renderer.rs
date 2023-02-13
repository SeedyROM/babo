use std::collections::HashMap;

use freetype::{Face, GlyphSlot, Library};
use nalgebra::{Orthographic3, Vector2, Vector3};

use crate::{gl, Shader, ShaderProgram, ShaderType, Texture};

impl From<Face> for Font {
    fn from(face: Face) -> Self {
        Self {
            face,
            glyphs: HashMap::new(),
        }
    }
}

pub struct Character {
    pub texture: Texture,
    pub bearing: Vector2<f32>,
    pub advance: f32,
}

impl TryFrom<&GlyphSlot> for Character {
    type Error = Box<dyn std::error::Error>;

    fn try_from(slot: &GlyphSlot) -> Result<Self, Self::Error> {
        let bitmap = slot.bitmap();

        let texture = Texture::new(
            bitmap.buffer(),
            bitmap.width() as u32,
            bitmap.rows() as u32,
            gl::RED,
            gl::RED,
            gl::CLAMP_TO_EDGE,
            gl::CLAMP_TO_EDGE,
            gl::LINEAR,
            gl::LINEAR,
        )?;
        let bearing = Vector2::new(slot.bitmap_left() as f32, slot.bitmap_top() as f32);
        let advance = slot.advance().x as f32;

        Ok(Self {
            texture,
            bearing,
            advance,
        })
    }
}

pub struct Font {
    pub face: Face,
    pub glyphs: HashMap<char, Character>,
}

impl Font {
    pub fn new(
        library: &Library,
        path: &str,
        size: u32,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let face = library.new_face(path, 0)?;

        face.set_pixel_sizes(0, size)?;

        let mut font = Self {
            face,
            glyphs: HashMap::new(),
        };

        font.load_default_glyphs()?;

        Ok(font)
    }

    pub fn load_glyph(&mut self, c: char) -> Result<&Character, Box<dyn std::error::Error>> {
        if !self.glyphs.contains_key(&c) {
            // Set the pixel store since we're loading a monochrome bitmap
            gl!(PixelStorei, gl::UNPACK_ALIGNMENT, 1)?;

            // Load the glyph
            self.face
                .load_char(c as usize, freetype::face::LoadFlag::RENDER)?;

            // Convert the glyph to a character
            let slot = self.face.glyph();
            let character = Character::try_from(slot)?;

            // Insert the character into the map
            self.glyphs.insert(c, character);
        }

        Ok(self.glyphs.get(&c).unwrap())
    }

    pub fn load_default_glyphs(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        for c in 0..128 {
            self.load_glyph(char::from_u32(c).unwrap())?;
        }

        Ok(())
    }
}

pub struct FontRenderer {
    pub library: Library,
    pub shader: ShaderProgram,
    pub vao: u32,
    pub vbo: u32,
}

impl FontRenderer {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let library = Library::init()?;

        let shader = ShaderProgram::from_shaders(&[
            Shader::from_source(
                ShaderType::Vertex,
                include_str!("../../../assets/shaders/font.vert"),
            )?,
            Shader::from_source(
                ShaderType::Fragment,
                include_str!("../../../assets/shaders/font.frag"),
            )?,
        ])?;

        let mut vao = 0;
        let mut vbo = 0;

        gl!(GenVertexArrays, 1, &mut vao)?;
        gl!(BindVertexArray, vao)?;

        gl!(GenBuffers, 1, &mut vbo)?;
        gl!(BindBuffer, gl::ARRAY_BUFFER, vbo)?;

        gl!(
            BufferData,
            gl::ARRAY_BUFFER,
            (6 * 4 * std::mem::size_of::<f32>()) as isize,
            std::ptr::null(),
            gl::DYNAMIC_DRAW,
        )?;

        gl!(
            VertexAttribPointer,
            0,
            4,
            gl::FLOAT,
            gl::FALSE,
            4 * std::mem::size_of::<f32>() as i32,
            std::ptr::null(),
        )?;
        gl!(EnableVertexAttribArray, 0)?;

        gl!(BindBuffer, gl::ARRAY_BUFFER, 0)?;
        gl!(BindVertexArray, 0)?;

        Ok(Self {
            library,
            shader,
            vao,
            vbo,
        })
    }

    pub fn draw(
        &mut self,
        font: &mut Font,
        text: &str,
        x: f32,
        y: f32,
        scale: f32,
        color: &Vector3<f32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.shader.use_program();

        self.shader
            .set_uniform_3f("textColor", (color.x, color.y, color.z))?;
        self.shader.set_uniform_mat4(
            "transform",
            &Orthographic3::new(0.0, 1280.0, 720.0, 0.0, -1.0, 1.0).into_inner(),
        )?;

        gl!(ActiveTexture, gl::TEXTURE0)?;
        gl!(BindVertexArray, self.vao)?;

        let mut x = x;
        let mut y = y;

        for c in text.chars() {
            let character = font.load_glyph(c)?;

            let xpos = x + character.bearing.x * scale;
            let ypos = y - (character.texture.height() as f32 - character.bearing.y) * scale;

            let w = character.texture.width() as f32 * scale;

            let h = character.texture.height() as f32 * scale;

            let vertices: [f32; 6 * 4] = [
                xpos,
                ypos + h,
                0.0,
                0.0,
                xpos,
                ypos,
                0.0,
                1.0,
                xpos + w,
                ypos,
                1.0,
                1.0,
                xpos,
                ypos + h,
                0.0,
                0.0,
                xpos + w,
                ypos,
                1.0,
                1.0,
                xpos + w,
                ypos + h,
                1.0,
                0.0,
            ];

            // Set blending.
            gl!(Enable, gl::BLEND)?;
            gl!(BlendFunc, gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA)?;

            gl!(BindTexture, gl::TEXTURE_2D, character.texture.id())?;
            gl!(BindBuffer, gl::ARRAY_BUFFER, self.vbo)?;
            gl!(
                BufferSubData,
                gl::ARRAY_BUFFER,
                0,
                vertices.len() as isize * std::mem::size_of::<f32>() as isize,
                vertices.as_ptr() as *const std::ffi::c_void,
            )?;
            gl!(BindBuffer, gl::ARRAY_BUFFER, 0)?;
            gl!(DrawArrays, gl::TRIANGLES, 0, 6)?;

            x += (character.advance as u32 >> 6) as f32 * scale;
            if c == '\n' {
                x = 0.0;
                y += character.texture.height() as f32 * scale;
            }
        }

        gl!(BindVertexArray, 0)?;
        gl!(BindTexture, gl::TEXTURE_2D, 0)?;

        Ok(())
    }

    pub fn load_font(&mut self, path: &str, size: u32) -> Result<Font, Box<dyn std::error::Error>> {
        Font::new(&self.library, path, size)
    }
}
