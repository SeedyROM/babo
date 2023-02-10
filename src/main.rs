use nalgebra::{Vector2, Vector3};
use sdl2::{event::Event, keyboard::Keycode};

use rendering::{Camera, SpriteRenderer, Texture, Window, WindowTrait};

mod rendering;
mod utils;

fn main() {
    let mut window = Window::new(1280, 720, "Babo Engine: v0.0.1").unwrap();

    let mut camera = Camera::new(1280.0, 720.0);

    let sprite_renderer = SpriteRenderer::new().unwrap();

    let texture = Texture::from_file("./assets/textures/awesome.png").unwrap();

    let babo_texture = Texture::from_file("./assets/textures/babo.png").unwrap();

    let mut position = Vector3::new(0.0, 0.0, 1.0);
    let mut rotation = 0.0;

    let big_boy_position = Vector3::new(128.0, 0.0, 1.0);
    let mut big_boy_rotation = 0.0;

    // Handle events.
    loop {
        if !window.running() {
            break;
        }

        for event in window.events() {
            match event {
                Event::Quit { .. } => {
                    window.stop();
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    window.stop();
                }
                _ => {}
            }
        }

        // Update some variables.
        position.x += 1.0;
        position.y += 0.4;
        rotation += 0.01;
        big_boy_rotation -= 0.005;

        // Set the camera at the center of sprite.
        camera.set_position(
            position.xy()
                + Vector2::new(
                    babo_texture.width() as f32 / 2.0,
                    babo_texture.height() as f32 / 2.0,
                ),
        );

        // Clear the screen.
        window.clear(f32::sin(rotation * std::f32::consts::PI * 2.0), 0.25, 0.25);

        // Render the sprite with the camera.
        sprite_renderer
            .draw(
                &texture,
                &camera.projection(),
                &camera.view(),
                &big_boy_position,
                &Vector2::new(720.0, 720.0),
                big_boy_rotation,
                &Vector3::new(1.0, 1.0, 1.0),
            )
            .unwrap();

        // Render the big boy sprite with the camera.
        sprite_renderer
            .draw(
                &babo_texture,
                &camera.projection(),
                &camera.view(),
                &position,
                &Vector2::new(babo_texture.width() as f32, babo_texture.height() as f32),
                rotation,
                &Vector3::new(1.0, 1.0, 1.0),
            )
            .unwrap();

        // Draw the screen.
        window.present();
    }
}
