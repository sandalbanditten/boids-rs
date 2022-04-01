use crate::boid::Boid;
use crate::flock::Flock;
use crate::keys::{key_pressed, key_released, Keybinds};
use crate::view;
use crate::window::window_resized;
use nannou::prelude::{App, Rect};

pub struct Model {
    pub flock: Vec<Boid>,
    pub keybinds: Keybinds,
    pub win_rect: Rect,
}

impl Model {
    // Our constructor
    pub fn new(app: &App) -> Model {
        // Creating the window
        let _window = app
            .new_window()
            .title("512 boids!")
            // Functions to call at certain event
            .key_pressed(key_pressed)
            .key_released(key_released)
            .resized(window_resized)
            .view(view)
            .build()
            .unwrap();

        // The window rect
        let win_rect = app.window_rect();

        // Our model is the state of our application, which can be accesed from all functions
        Model {
            flock: Flock::new_flock(win_rect, 1024),
            keybinds: Keybinds::new(),
            win_rect,
        }
    }
}
