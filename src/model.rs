use crate::boid::*;
use crate::keys::{key_pressed, key_released};
use crate::view;
use crate::window::*;
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
            // TODO: Title here should reflect number of boids given with CLI args
            .title("512 boids!")
            // Functions to call at certain event
            .key_pressed(key_pressed)
            .key_released(key_released)
            .resized(window_resized)
            .view(view)
            .build()
            .unwrap();

        let mut flock: Vec<Boid> = Vec::new();
        let win_rect: Rect = app.window_rect();
        for _ in 0..512 {
            flock.push(Boid::new(
                // Position vector
                Vec2::new(
                    // Random, inside the rect
                    random_range(win_rect.left(), win_rect.right()),
                    random_range(win_rect.bottom(), win_rect.top()),
                ),
                // Velocity vector - random, but changed to max_speed
                Vec2::new(random_range(-1.0, 1.0), random_range(-1.0, 1.0)).clamp_length_max(0.075),
                // Acceleration vector
                Vec2::new(0.0, 0.0),
                win_rect,
            ));
        }

        // Our model is the state of our application, which can be accesed from all functions
        Model { flock }
    }
}
