use crate::boid::Boid;
use crate::flock::Flock;
use crate::keys::{key_pressed, key_released, Keybinds};
use crate::view;
use crate::window::window_resized;
use nannou::prelude::App;
use nannou::prelude::Rect;

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

        // Our model is the state of our application, which can be accesed from all functions
        Model {
            flock: Flock::new_flock(app.window_rect(), 512),
            keybinds: Keybinds {
                highlight_all: false,
                highlight_first: false,
                show_help_menu: true,
                show_current_values: false,
                highlight_all_is_pressed: false,
                highlight_first_is_pressed: false,
                show_help_menu_is_pressed: false,
                show_current_values_is_pressed: false,
            },
            win_rect: app.window_rect(),
        }
    }
}
