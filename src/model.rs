use crate::boid::Boid;
use crate::flock::Flock;
use crate::keys::{key_pressed, key_released, Keybinds};
use crate::view;
use crate::window::resized;

use nannou::prelude::{App, Draw, Rect};

const INITIAL_BOIDS: usize = 1024;

pub struct Model {
    pub flock: Vec<Boid>,
    pub keybinds: Keybinds,
    pub win_rect: Rect,
    pub draw: Draw,
}

impl Model {
    // Our constructor
    pub fn new(app: &App) -> Self {
        // Creating the window
        let _window = app
            .new_window()
            .title(format!("{INITIAL_BOIDS} boids!"))
            // Functions to call at certain event
            .key_pressed(key_pressed)
            .key_released(key_released)
            .resized(resized)
            .view(view)
            .build()
            .expect("Unable to build the app");

        // The window rect
        let win_rect = app.window_rect();

        // Our model is the state of our application, which can be accessed from all functions
        Self {
            flock: Flock::new_flock(win_rect, INITIAL_BOIDS),
            keybinds: Keybinds::default(),
            win_rect,
            draw: app.draw(),
        }
    }
}
