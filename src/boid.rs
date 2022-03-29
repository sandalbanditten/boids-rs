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
            color: Color::new(
                1.0, 1.0, 1.0, 1.0,
            ),
            diameter: 0.3,
            radius: 0.3 / 2.0,
            perception_radius: 2.5,
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

    pub fn flock(&mut self, flock: &Vec<Boid>, win_rect: Rect) {
        // The three rules
        let alignment = self.align(flock);
        let cohesion = self.cohere(flock);
        let separation = self.separate(flock);
        self.acceleration = alignment + cohesion + separation;

        // Update colors based on pos, vel, and acc
        self.update_color(win_rect);
        // Update velocity and position
        self.update(win_rect);
    }

    // Updating the position and velocity of the boid
    fn update(&mut self, win_rect: Rect) {
        self.position += self.velocity;
        self.velocity += self.acceleration;

        // Making the speed at most max_speed
        self.velocity.clamp_length(self.max_speed, self.max_speed);

        // Reset the acceleration
        self.acceleration = Vec2::ZERO;

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

    // Update the color of the boid, based on pos, vel and acc
    fn update_color(&mut self, win_rect: Rect) {
        self.color = Color::new(
            // Map left to right
            map(self.position.x, win_rect.left(), win_rect.right(), 0.0, 1.0),
            // Map bottom to top
            map(self.position.y, win_rect.bottom(), win_rect.top(), 0.0, 1.0),
            // Map the direction of the bird
            map(self.velocity.angle(), 0.0, f32::TAU(), 0.5, 1.0),
            1.0,
        )
    }

    // TODO: Merge the three functions, for effeciency
    fn align(&mut self, flock: &Vec<Boid>) -> Vec2 {
        // Compute the average steering
        let mut average_steering = Vec2::ZERO;
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

    fn cohere(&mut self, flock: &Vec<Boid>) -> Vec2 {
        // The final vector to steer towards
        let mut steering = Vec2::ZERO;

        // Compute the average location
        let mut average_location = Vec2::ZERO;
        let mut total = 0;
        for other in flock.iter() {
            let distance = self.position.distance(other.position);
            // Only count the ones within perception_radius and the ones that arent itself
            if distance < self.perception_radius && self != other {
                average_location += other.position;
                total += 1;
            }
        }

        // Only change self if there is actually any boids nearby
        if total > 0 {
            // Divides the average by a vector with the values of the length of the part of flock within perception
            // The average location
            average_location /= Vec2::new(total as f32, total as f32);
            average_location -= self.position;
            steering = average_location.clamp_length(self.max_speed, self.max_speed);
            steering -= self.velocity;
            // Only get affected by the other boids by a certain amount
            steering = average_location.clamp_length_max(self.max_force);
        }
        steering
    }

    fn separate(&mut self, flock: &Vec<Boid>) -> Vec2 {
        // The final vector to steer towards
        let mut steering = Vec2::ZERO;

        // Compute the average location
        let mut average_location = Vec2::ZERO;
        let mut total = 0;
        for other in flock {
            let distance = self.position.distance(other.position);
            // Only count the ones within perception_radius and the ones that arent itself
            if distance < self.perception_radius && self != other {
                average_location += other.position;
                total += 1;
            }
        }

        // Only change self if there is actually any boids nearby
        if total > 0 {
            // Divides the average by a vector with the values of the length of the part of flock within perception
            // The average location
            average_location /= Vec2::new(total as f32, total as f32);
            average_location -= self.position;
            steering = average_location.clamp_length(self.max_speed, self.max_speed);
            steering -= self.velocity;
            // Only get affected by the other boids by a certain amount
            steering = average_location.clamp_length_max(self.max_force);
        }
        steering
    }
}
