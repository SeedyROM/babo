#![allow(unused)]

use image::{GenericImageView, ImageError};

pub struct Texture {
    id: u32,
    width: u32,
    height: u32,
    internal_format: u32,
    image_format: u32,
    wrap_s: u32,
    wrap_t: u32,
    filter_min: u32,
    filter_max: u32,
}

impl Default for Texture {
    fn default() -> Self {
        Self {
            id: 0,
            width: 0,
            height: 0,
            internal_format: gl::RGBA,
            image_format: gl::RGBA,
            wrap_s: gl::REPEAT,
            wrap_t: gl::REPEAT,
            filter_min: gl::LINEAR,
            filter_max: gl::LINEAR,
        }
    }
}

impl Texture {
    pub fn new(
        width: u32,
        height: u32,
        internal_format: u32,
        image_format: u32,
        wrap_s: u32,
        wrap_t: u32,
        filter_min: u32,
        filter_max: u32,
    ) -> Self {
        let mut id = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                internal_format as i32,
                width as i32,
                height as i32,
                0,
                image_format,
                gl::UNSIGNED_BYTE,
                std::ptr::null(),
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, wrap_s as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, wrap_t as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, filter_min as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, filter_max as i32);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }

        Texture {
            id,
            width,
            height,
            internal_format,
            image_format,
            wrap_s,
            wrap_t,
            filter_min,
            filter_max,
        }
    }

    pub fn from_file(path: &str) -> Result<Self, ImageError> {
        let image = image::open(path)?;
        let (width, height) = image.dimensions();
        let data = image.as_bytes();

        let mut id = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                width as i32,
                height as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                data.as_ptr() as *const _,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);

            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                gl::LINEAR_MIPMAP_LINEAR as i32,
            );

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            gl::BindTexture(gl::TEXTURE_2D, 0);

            Ok(Texture {
                id,
                width,
                height,
                internal_format: gl::RGBA as u32,
                image_format: gl::RGBA,
                wrap_s: gl::REPEAT,
                wrap_t: gl::REPEAT,
                filter_min: gl::LINEAR,
                filter_max: gl::LINEAR,
            })
        }
    }

    pub fn generate_mipmaps(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }

    pub fn set_wrap_s(&mut self, wrap_s: u32) {
        self.wrap_s = wrap_s;
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, wrap_s as i32);
        }
    }

    pub fn set_wrap_t(&mut self, wrap_t: u32) {
        self.wrap_t = wrap_t;
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, wrap_t as i32);
        }
    }

    pub fn set_filter_min(&mut self, filter_min: u32) {
        self.filter_min = filter_min;
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, filter_min as i32);
        }
    }

    pub fn set_filter_max(&mut self, filter_max: u32) {
        self.filter_max = filter_max;
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, filter_max as i32);
        }
    }

    pub fn set_filter(&mut self, filter_min: u32, filter_max: u32) {
        self.filter_min = filter_min;
        self.filter_max = filter_max;
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, filter_min as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, filter_max as i32);
        }
    }

    pub fn set_wrap(&mut self, wrap_s: u32, wrap_t: u32) {
        self.wrap_s = wrap_s;
        self.wrap_t = wrap_t;
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, wrap_s as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, wrap_t as i32);
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}
