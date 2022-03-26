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
            .title("Template!")
            // Functions to call
            .key_pressed(key_pressed)
            .key_released(key_released)
            .view(view)
            .build()
            .unwrap();

        let mut flock: Vec<Boid> = Vec::new();
        for _ in 0..256 {
            flock.push(Boid::new(
                Vec2::new(random_range(-5.0, 5.0), random_range(-5.0, 5.0)),
                Vec2::new(random_range(-0.2, 0.2), random_range(-0.2, 0.2)),
                Vec2::new(0.0, 0.0),
            ))
        }

        // Our model is the state of our application, which can be accesed from all functions
        Model { flock }
    }
}
