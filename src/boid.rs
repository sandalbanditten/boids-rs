use crate::color::Color;
use crate::math::map;
use nannou::prelude::{random_range, Draw, Point2, Rect, Vec2, Vec2Angle};

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
    // How far the boid can "see"
    perception_radius: f32,
    // These three modifiers get applied to the alignment etc. to scale it
    alignment_modifier: f32,
    cohesion_modifier: f32,
    separation_modifier: f32,
}

impl Boid {
    // Constructor //
    pub fn new(position: Vec2, velocity: Vec2) -> Self {
        Boid {
            position,
            // Sets the length of the vector to 0.075
            velocity: velocity.normalize().clamp_length(0.075, 0.075),
            // Use the default implementation for the rest of the boid
            ..Default::default()
        }
    }

    // For showing boids //
    // Shows the boid to the screen, as a triangle, pointing in the same direction as the boid
    pub fn show(&self, draw: &Draw) {
        draw.tri()
            .xy(self.position)
            // A triangle pointing to the right - so it has an angle of zero degrees
            // Basically looks like this:
            /*
             *     *
             *         *
             *     *
             */
            .points(
                Point2::new(self.diameter / 2.0, 0.0),
                Point2::new(-self.diameter / 2.0, -self.diameter / 2.0),
                Point2::new(-self.diameter / 2.0, self.diameter / 2.0),
            )
            .w_h(self.diameter, self.diameter)
            // Set its angle to the boids velocity angle - where the boid is facing
            .rotate(self.velocity.angle())
            .rgba(self.color.r, self.color.g, self.color.b, self.color.a);
    }

    // Draws a transparent circle at the boids position, with a radius equal to the boids
    // perception_radius
    pub fn show_perception(&self, draw: &Draw, mut alpha: f32) {
        // making sure the alpha is between 0.0 and 1.0 - this might happen internally in the function, though this is not discernable from the source code
        alpha = alpha.clamp(0.0, 1.0);
        draw.ellipse()
            .w_h(self.perception_radius * 2.0, self.perception_radius * 2.0)
            .xy(self.position)
            .rgba(1.0, 1.0, 1.0, alpha);
    }

    // The three rules //
    // The main flocking function - calls the three rules, and updates the boids with color and
    // movement
    pub fn flock(&mut self, flock: &[Boid], win_rect: Rect) {
        // Changed from &Vec<Boid> to &[Boid], from vector type to slice type
        // The three rules
        let alignment = self.align(flock) * self.alignment_modifier;
        let cohesion = self.cohere(flock) * self.cohesion_modifier;
        let separation = self.separate(flock) * self.separation_modifier;

        // Updating the acceleration
        self.acceleration += alignment;
        self.acceleration += cohesion;
        self.acceleration += separation;

        // Update velocity and position - and resetting acceleration
        self.update(win_rect);
        // Update colors based on pos, vel, and acc - updating after self.update() is important
        self.update_color(win_rect);
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
        if self.position.x < boundary_rect.left() + self.diameter / 2.0 {
            self.position.x = boundary_rect.right() - self.diameter / 2.0;
        }
        if self.position.x > boundary_rect.right() - self.diameter / 2.0 {
            self.position.x = boundary_rect.left() + self.diameter / 2.0;
        }
        if self.position.y < boundary_rect.bottom() + self.diameter / 2.0 {
            self.position.y = boundary_rect.top() - self.diameter / 2.0;
        }
        if self.position.y > boundary_rect.top() - self.diameter / 2.0 {
            self.position.y = boundary_rect.bottom() + self.diameter / 2.0;
        }
    }

    // The three separate methods for the three rules
    fn align(&self, flock: &[Boid]) -> Vec2 {
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

    fn cohere(&self, flock: &[Boid]) -> Vec2 {
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

    fn separate(&self, flock: &[Boid]) -> Vec2 {
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

    // Functions for getting attributes //
    // Returns the perception radius of the boid
    pub fn get_perception(&self) -> f32 {
        self.perception_radius
    }

    // Returns the diameter of the boid
    pub fn get_diameter(&self) -> f32 {
        self.diameter
    }

    // Returns the max speed of the boid
    pub fn get_max_speed(&self) -> f32 {
        self.max_speed
    }

    // Returns the max force of the boid
    pub fn get_max_force(&self) -> f32 {
        self.max_force
    }

    // Returns the alignment modifier of the boid
    pub fn get_alignment_modifier(&self) -> f32 {
        self.alignment_modifier
    }

    // Returns the cohesion modifier of the boid
    pub fn get_cohesion_modifier(&self) -> f32 {
        self.cohesion_modifier
    }

    // Returns the separation modifier of the boid
    pub fn get_separation_modifier(&self) -> f32 {
        self.separation_modifier
    }

    // Functions for changing attributes //
    // Change the perception of the boid
    pub fn change_perception(&mut self, multiplier: f32) {
        self.perception_radius *= multiplier;
    }

    // Change the size of the boid
    pub fn change_diameter(&mut self, multiplier: f32) {
        self.diameter *= multiplier;
    }

    // Changes the max speed of the boid
    pub fn change_max_speed(&mut self, multiplier: f32) {
        self.max_speed *= multiplier;
    }

    // Changes the max force of the boid
    pub fn change_max_force(&mut self, multiplier: f32) {
        self.max_force *= multiplier;
    }

    // Changes the alignment modifier of the boid
    pub fn change_alignment_modifier(&mut self, multiplier: f32) {
        self.alignment_modifier *= multiplier;
    }

    // Changes the cohesion modifier of the boid
    pub fn change_cohesion_modifier(&mut self, multiplier: f32) {
        self.cohesion_modifier *= multiplier;
    }

    // Changes the separation modifier of the boid
    pub fn change_separation_modifier(&mut self, multiplier: f32) {
        self.separation_modifier *= multiplier;
    }

    // Changes the position of the boid
    pub fn change_position(&mut self, new_position: Vec2) {
        self.position = new_position;
    }

    // Changes the velocity of the boid
    pub fn change_velocity(&mut self, new_velocity: Vec2) {
        self.velocity = new_velocity;
    }

    // Update the color of the boid, based on pos, vel and acc
    fn update_color(&mut self, win_rect: Rect) {
        // The lower and upper possible rgb values for the boids
        // Having them be != 0.0 or 1.0 means that there will be no fully black and no fully white
        // boids
        let lower: f32 = 0.2;
        let upper: f32 = 0.8;
        // Change the color
        // R - velocity
        // G - X-position
        // B - Y-position
        self.color = Color::new(
            map(
                self.velocity.length(),
                0.0,
                self.get_max_speed(),
                lower,
                upper,
            ),
            // Map left to right
            map(
                self.position.x,
                win_rect.left(),
                win_rect.right(),
                lower,
                upper,
            ),
            // Map bottom to top
            map(
                self.position.y,
                win_rect.bottom(),
                win_rect.top(),
                lower,
                upper,
            ),
            1.0,
        )
    }
}

// The default implementation for boids - the middle of the screen etc.
impl Default for Boid {
    fn default() -> Self {
        Boid {
            position: Vec2::new(0.0, 0.0),
            velocity: Vec2::new(random_range(-0.1, 0.1), random_range(-0.1, 0.1)),
            acceleration: Vec2::new(0.0, 0.0),
            max_speed: 0.075,
            max_force: 0.0005,
            color: Color::new(1.0, 1.0, 1.0, 1.0),
            diameter: 0.3,
            perception_radius: 2.5,
            alignment_modifier: 1.0,
            cohesion_modifier: 1.0,
            separation_modifier: 0.9,
        }
    }
}
