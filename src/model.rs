use crate::boid::*;
use crate::keys::{key_pressed, key_released};
use crate::view;
use nannou::prelude::*;

pub struct Model {
    pub flock: Vec<Boid>,
}

impl Model {
    // Our constructor
    pub fn new(app: &App) -> Model {
        // Creating the window
        let _window = app
            .new_window()
            .title("boids!")
            // Functions to call
            .key_pressed(key_pressed)
            .key_released(key_released)
            .view(view)
            .build()
            .unwrap();

        let mut flock: Vec<Boid> = Vec::new();
        let win_rect: Rect = app.window_rect();
        for _ in 0..256 {
            flock.push(Boid::new(
                    // Position vector
                Vec2::new(
                    random_range(win_rect.left(), win_rect.right()),
                    random_range(win_rect.bottom(), win_rect.top()),
                ),
                // Velocity vector
                Vec2::new(random_range(-0.1, 0.1), random_range(-0.1, 0.1)),
                // Acceleration vector
                Vec2::new(0.0, 0.0),
            ))
        }

        // Our model is the state of our application, which can be accesed from all functions
        Model { flock }
    }
}
