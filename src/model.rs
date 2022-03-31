use crate::boid::Boid;
use crate::flock::Flock;
use crate::keys::{key_pressed, key_released, Keybinds};
use crate::view;
use crate::window::window_resized;
use nannou::prelude::App;

pub struct Model {
    pub flock: Vec<Boid>,
    pub keybinds: Keybinds,
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

        // Our model is the state of our application, which can be accesed from all functions
        Model {
            flock: Flock::new_flock(app.window_rect()),
            keybinds: Keybinds {
                highlight_all: false,
                highlight_first: false,
                show_help_menu: true,
                show_current_values: false,
            },
        }
    }
}
