use crate::boid::Boid;
use nannou::prelude::{random_range, Rect, Vec2};

pub struct Flock;

impl Flock {
    pub fn new_flock(boundary_rect: Rect, flock_size: usize) -> Vec<Boid> {
        let mut flock: Vec<Boid> = Vec::new();
        for _ in 0..flock_size {
            flock.push(Boid::new(
                // Position vector
                Vec2::new(
                    // Random, inside the rect
                    random_range(boundary_rect.left(), boundary_rect.right()),
                    random_range(boundary_rect.bottom(), boundary_rect.top()),
                ),
                // Velocity vector - random, but clamped to max_speed
                Vec2::new(random_range(-1.0, 1.0), random_range(-1.0, 1.0)).clamp_length_max(0.075),
            ));
        }
        flock
    }
}
