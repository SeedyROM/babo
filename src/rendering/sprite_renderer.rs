use nalgebra::{Matrix4, Vector2, Vector3};

pub trait SpriteRendererTrait {
    type Texture;

    fn new() -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized;

    fn draw(
        &self,
        texture: &Self::Texture,
        projection: &Matrix4<f32>,
        view: &Matrix4<f32>,
        position: &Vector3<f32>,
        size: &Vector2<f32>,
        rotation: f32,
        color: &Vector3<f32>,
    ) -> Result<(), Box<dyn std::error::Error>>;
}
