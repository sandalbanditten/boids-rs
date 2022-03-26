use crate::keys::{key_pressed, key_released};
use crate::view;
use nannou::prelude::*;

pub struct Model {}

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
        // Our model is the state of our application, which can be accesed from all functions
        Model {}
    }
}
