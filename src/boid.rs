use crate::color::*;
use crate::math::map;
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
    boundary_rect: Rect,
}

impl Boid {
    pub fn new(position: Vec2, velocity: Vec2, acceleration: Vec2, win_rect: Rect) -> Boid {
        Boid {
            position,
            // Sets the length of the vector to 0.1
            velocity: velocity.normalize().clamp_length(0.1, 0.1),
            acceleration,
            max_speed: 0.075,
            max_force: 0.0005,
            color: Color::new(1.0, 1.0, 1.0, 1.0),
            diameter: 0.3,
            radius: 0.3 / 2.0,
            perception_radius: 2.5,
            boundary_rect: win_rect,
        }
    }

    // Shows the boid to the screen
    pub fn show(&self, draw: &Draw) {
        draw.tri()
            .xy(self.position)
            // A triangle pointing to the right - so it has an angle of zero degrees
            .points(
                Point2::new(self.diameter, 0.0),
                Point2::new(-self.diameter, -self.diameter),
                Point2::new(-self.diameter, self.diameter),
            )
            .w_h(self.diameter, self.diameter)
            // Set its angle to the boids velocity
            .rotate(self.velocity.angle())
            .rgba(self.color.r, self.color.g, self.color.b, self.color.a);
    }

    pub fn show_perception(&self, draw: &Draw) {
        draw.ellipse()
            .w_h(self.perception_radius * 2.0, self.perception_radius * 2.0)
            .xy(self.position)
            .rgba8(255, 255, 255, 1);
    }

    // Changed from &Vec<Boid> to &[Boid], so it also works with arrays
    // The main flocking function - calls the three rules, and updates the boids with color and
    // movement
    pub fn flock(&mut self, flock: &[Boid]) {
        // The three rules
        let alignment = self.align(flock);
        let cohesion = self.cohere(flock);
        let separation = self.separate(flock);

        // Updating the acceleration
        self.acceleration += alignment;
        self.acceleration += cohesion;
        self.acceleration += separation;

        // Update colors based on pos, vel, and acc
        self.update_color();
        // Update velocity and position
        self.update(self.boundary_rect);
    }

    // Updating the position and velocity of the boid
    fn update(&mut self, boundary_rect: Rect) {
        self.position += self.velocity;
        self.velocity += self.acceleration;

        // Making the speed at most max_speed
        self.velocity = self.velocity.clamp_length_max(self.max_speed);

        // Reset the acceleration
        self.acceleration = Vec2::ZERO;

        // Check if stuff is inside bounds
        if self.position.x < boundary_rect.left() + self.radius {
            self.position.x = boundary_rect.right() - self.radius;
        }
        if self.position.x > boundary_rect.right() - self.radius {
            self.position.x = boundary_rect.left() + self.radius;
        }
        if self.position.y < boundary_rect.bottom() + self.radius {
            self.position.y = boundary_rect.top() - self.radius;
        }
        if self.position.y > boundary_rect.top() - self.radius {
            self.position.y = boundary_rect.bottom() + self.radius;
        }
    }

    // Functions for changing attributes
    pub fn change_position(&mut self, new_position: Vec2) {
        self.position = new_position;
    }

    pub fn change_velocity(&mut self, new_velocity: Vec2) {
        self.velocity = new_velocity;
    }

    pub fn change_boundary(&mut self, new_boundary: Rect) {
        self.boundary_rect = new_boundary;
    }

    // Update the color of the boid, based on pos, vel and acc
    fn update_color(&mut self) {
        // The lower and upper possible rgb values for the boids
        // Having them be != 0.0 or 1.0 means that there will be no fully black and no fully white
        // boids
        let lower = 0.2;
        let upper = 0.8;
        self.color = Color::new(
            // Map left to right
            map(
                self.position.x,
                self.boundary_rect.left(),
                self.boundary_rect.right(),
                lower,
                upper,
            ),
            // Map bottom to top
            map(
                self.position.y,
                self.boundary_rect.bottom(),
                self.boundary_rect.top(),
                lower,
                upper,
            ),
            1.0,
            1.0,
        )
    }

    // TODO: Merge the three functions, for efficiency
    fn align(&mut self, flock: &[Boid]) -> Vec2 {
        // Compute the average steering
        let mut steering = Vec2::ZERO;
        let mut total = 0;
        for other in flock.iter() {
            let distance = self.position.distance(other.position);
            // Only count the ones within perception_radius and the ones that arent itself
            if distance < self.perception_radius && self != other {
                steering += other.velocity;
                total += 1;
            }
        }
        if total > 0 {
            // Divides the average by a vector with the values of the length of the part of flock within perception
            // The average steering
            steering /= Vec2::new(total as f32, total as f32);
            // Set the length of the vector to the boids max speed
            steering = steering.clamp_length(self.max_speed, self.max_speed);
            steering -= self.velocity;
            // Only get affected by the other boids by a certain amount
            steering = steering.clamp_length_max(self.max_force);
        }
        steering
    }

    fn cohere(&mut self, flock: &[Boid]) -> Vec2 {
        // Compute the average location
        let mut steering = Vec2::ZERO;
        let mut total = 0;
        for other in flock.iter() {
            let distance = self.position.distance(other.position);
            // Only count the ones within perception_radius and the ones that arent itself
            if distance < self.perception_radius && self != other {
                steering += other.position;
                total += 1;
            }
        }

        // Only change self if there is actually any boids nearby
        if total > 0 {
            // Divides the average by a vector with the values of the length of the part of flock within perception
            steering /= Vec2::new(total as f32, total as f32);
            steering -= self.position;
            // Set the length of the vector to the boids max speed
            steering = steering.clamp_length(self.max_speed, self.max_speed);
            steering -= self.velocity;
            // Only get affected by the other boids by a certain amount
            steering = steering.clamp_length_max(self.max_force);
        }
        steering
    }

    fn separate(&mut self, flock: &[Boid]) -> Vec2 {
        // The final vector to steer towards
        let mut steering = Vec2::ZERO;
        let mut total = 0;

        for other in flock {
            let distance = self.position.distance(other.position);
            // Only count the ones within perception_radius and the ones that arent itself
            if distance < self.perception_radius && self != other {
                let mut difference = self.position - other.position;
                // Make the effect stronger the closer the boids are together
                difference /= distance * distance;
                // Add the difference between positions
                steering += difference;
                total += 1;
            }
        }

        // Only change self if there is actually any boids nearby
        if total > 0 {
            // Divides the average by a vector with the values of the length of the part of flock within perception
            steering /= Vec2::new(total as f32, total as f32);
            // Set the length of the vector to the boids max speed
            steering = steering.clamp_length(self.max_speed, self.max_speed);
            steering -= self.velocity;
            // Only get affected by the other boids by a certain amount
            steering = steering.clamp_length_max(self.max_force);
        }
        steering
    }
}
