use crate::color::*;
use nannou::prelude::*;

// So we can compare boids using ==
#[derive(PartialEq, Clone)]
pub struct Boid {
    position: Vec2,
    velocity: Vec2,
    acceleration: Vec2,
    max_speed: f32,
    max_force: f32,
    color: Color,
    diameter: f32,
    radius: f32,
    perception_radius: f32,
}

impl Boid {
    pub fn new(position: Vec2, velocity: Vec2, acceleration: Vec2) -> Boid {
        Boid {
            position,
            // Sets the length of the vector to 0.1
            velocity: velocity.normalize().clamp_length(0.1, 0.1),
            acceleration,
            max_speed: 0.1,
            max_force: 0.0005,
            // Random color
            color: Color::new(
                random_range(0, 255),
                random_range(0, 255),
                random_range(0, 255),
                255,
            ),
            diameter: 0.3,
            radius: 0.3 / 2.0,
            perception_radius: 2.5,
        }
    }

    // Shows the boid to the screen
    pub fn show(&self, draw: &Draw) {
        draw.ellipse()
            .w_h(self.diameter, self.diameter)
            .xy(self.position)
            .rgba8(self.color.r, self.color.g, self.color.b, self.color.a);
    }

    pub fn show_perception(&self, draw: &Draw) {
        draw.ellipse()
            .w_h(self.perception_radius * 2.0, self.perception_radius * 2.0)
            .xy(self.position)
            .rgba8(255, 255, 255, 1);
    }

    // Updating the position and velocity of the boid
    pub fn update(&mut self, win_rect: Rect) {
        // Making the speed at most max_speed
        self.velocity.clamp_length_max(self.max_speed);

        self.position += self.velocity;
        self.velocity += self.acceleration;

        // Check if stuff is inside bounds
        if self.position.x < win_rect.left() + self.radius {
            self.position.x = win_rect.right() - self.radius;
        }
        if self.position.x > win_rect.right() - self.radius {
            self.position.x = win_rect.left() + self.radius;
        }
        if self.position.y < win_rect.bottom() + self.radius {
            self.position.y = win_rect.top() - self.radius;
        }
        if self.position.y > win_rect.top() - self.radius {
            self.position.y = win_rect.bottom() + self.radius;
        }
    }

    pub fn flock(&mut self, flock: &Vec<Boid>) {
        let alignment = self.align(flock);
        self.acceleration = alignment;
    }

    // Aligns this boids steering with the average of the boids within perception_ranges steeruing
    fn align(&mut self, flock: &Vec<Boid>) -> Vec2 {
        // Compute the average steering
        let mut average_steering = Vec2::new(0.0, 0.0);
        let mut total = 0;
        for other in flock.iter() {
            let distance = self.position.distance(other.position);
            // Only count the ones within perception_radius and the ones that arent itself
            if distance < self.perception_radius && self != other {
                average_steering += other.velocity;
                total += 1;
            }
        }
        if total > 0 {
            // Divides the average by a vector with the values of the length of the part of flock within perception
            // The average steering
            average_steering /= Vec2::new(total as f32, total as f32);
            // Setting the  vector to a specific length
            average_steering = average_steering.clamp_length(self.max_speed, self.max_speed);
            average_steering -= self.velocity;
            // Only get affected by the other boids by a certain amount
            average_steering = average_steering.clamp_length_max(self.max_force);
        }
        average_steering
    }
}
