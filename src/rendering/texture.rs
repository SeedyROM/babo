#![allow(unused)]

pub trait TextureTrait {
    fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized;

    fn width(&self) -> u32;
    fn height(&self) -> u32;
}
