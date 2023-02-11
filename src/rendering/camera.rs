#![allow(unused)]

use nalgebra::{Matrix4, Orthographic3, Vector2, Vector3};

use crate::WindowTrait;

pub struct Camera {
    projection: Orthographic3<f32>,
    screen: Vector2<f32>,
    position: Vector2<f32>,
    zoom: Vector2<f32>,
    rotation: f32,
}

impl Camera {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            screen: Vector2::new(width, height),
            projection: Orthographic3::new(0.0, width, height, 0.0, -1.0, 1.0),
            position: Vector2::new(0.0, 0.0),
            zoom: Vector2::new(1.0, 1.0),
            rotation: 0.0,
        }
    }

    pub fn view(&self) -> Matrix4<f32> {
        let mut view = Matrix4::identity();

        // Center the camera.
        view *=
            Matrix4::new_translation(&Vector3::new(self.screen.x / 2.0, self.screen.y / 2.0, 0.0));

        // Set the position.
        view *= Matrix4::new_translation(&Vector3::new(-self.position.x, -self.position.y, 0.0));

        // Rotate around the origin.
        view *= Matrix4::new_translation(&Vector3::new(-self.position.x, -self.position.y, 0.0));
        view *= Matrix4::new_rotation(&Vector3::z() * self.rotation);
        view *= Matrix4::new_translation(&Vector3::new(self.position.x, self.position.y, 0.0));

        // Zoom around the center of the screen.
        view *=
            Matrix4::new_translation(&Vector3::new(self.screen.x / 2.0, self.screen.y / 2.0, 0.0));
        view *= Matrix4::new_nonuniform_scaling(&Vector3::new(self.zoom.x, self.zoom.y, 1.0));
        view *= Matrix4::new_translation(&Vector3::new(
            -self.screen.x / 2.0,
            -self.screen.y / 2.0,
            0.0,
        ));

        view
    }

    pub fn projection(&self) -> &Matrix4<f32> {
        &self.projection.as_matrix()
    }

    pub fn set_position(&mut self, position: Vector2<f32>) {
        self.position = position;
    }

    pub fn set_zoom(&mut self, zoom: Vector2<f32>) {
        self.zoom = zoom;
    }

    pub fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;
    }

    pub fn set_screen(&mut self, width: f32, height: f32) {
        self.screen = Vector2::new(width, height);
        self.projection = Orthographic3::new(0.0, width, height, 0.0, -1.0, 1.0);
    }
}

impl<'a, T> From<&T> for Camera
where
    T: WindowTrait<'a>,
{
    fn from(window: &T) -> Self {
        Self::new(window.width() as f32, window.height() as f32)
    }
}
