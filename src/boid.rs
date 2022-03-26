use crate::color::*;
use nannou::prelude::*;

pub struct Boid {
    position: Vec2,
    velocity: Vec2,
    acceleration: Vec2,
    color: Color,
}

impl Boid {
    pub fn new(position: Vec2, velocity: Vec2, acceleration: Vec2) -> Boid {
        Boid {
            position,
            velocity,
            acceleration,
            color: Color::new(
                random_range(0, 255),
                random_range(0, 255),
                random_range(0, 255),
                255,
            ),
        }
    }
    pub fn show(&self, draw: &Draw) {
        draw.ellipse().w_h(0.2, 0.2).xy(self.position).rgba8(
            self.color.r,
            self.color.g,
            self.color.b,
            self.color.a,
        );
    }
    pub fn update(&mut self) {
        self.position += self.velocity;
        self.velocity += self.acceleration;
    }
}
