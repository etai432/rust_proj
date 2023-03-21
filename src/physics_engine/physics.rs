use macroquad::prelude::*;
use strum_macros::EnumIter;
pub struct Circle {
    pub color: Color,
    pub position_x: f64,
    pub position_y: f64,
    pub velocity: (f64, f64),
    pub acceleration: (f64, f64),
    pub radius: f64,
    pub bounciness: f64,
}

impl Circle {
    pub fn new(
        color: Color,
        position: (f64, f64),
        velocity: (f64, f64),
        gravity: (f64, f64),
        radius: f64,
    ) -> Self {
        Circle {
            color: color,
            position_x: position.0,
            position_y: position.1,
            velocity: velocity,
            acceleration: gravity,
            radius: radius,
            bounciness: 90.0,
        }
    }
    pub fn update_position(&mut self, dt: f64) {
        self.position_x += self.velocity.0 * dt + 0.5 * self.acceleration.0 * dt * dt;
        self.position_y += self.velocity.1 * dt + 0.5 * self.acceleration.1 * dt * dt;
        if self.position_x < self.radius {
            self.position_x = self.radius * 2.0 - self.position_x;
            self.velocity.0 *= -1.0 * self.bounciness / 100.0;
        }
        if self.position_x > screen_width() as f64 - self.radius {
            self.position_x = (screen_width() as f64 - self.radius) * 2.0 - self.position_x;
            self.velocity.0 *= -1.0 * self.bounciness / 100.0;
        }
        if self.position_y < self.radius {
            self.position_y = self.radius * 2.0 - self.position_y;
            self.velocity.1 *= -1.0 * self.bounciness / 100.0;
        }
        if self.position_y > screen_height() as f64 - self.radius {
            self.position_y = (screen_height() as f64 - self.radius) * 2.0 - self.position_y;
            self.velocity.1 *= -1.0 * self.bounciness / 100.0;
        }
        self.velocity.0 += self.acceleration.0 * dt;
        self.velocity.1 += self.acceleration.1 * dt;
    }
}

pub fn is_colliding(circle1: &Circle, circle2: &Circle) -> bool {
    ((circle1.position_x - circle2.position_x).powf(2.0)
        + (circle1.position_y - circle2.position_y).powf(2.0))
    .sqrt()
        < circle1.radius + circle2.radius
}

pub fn collision(circles: &mut Vec<Circle>, indexes: (usize, usize)) {}

#[derive(EnumIter, Debug)]
pub enum Gravity {
    Earth,
    Moon,
    Mars,
    Mercury,
    Venus,
    Pluto,
    Sun,
    None,
}

impl Gravity {
    pub fn get_gravity(self) -> f64 {
        //find screen size
        match self {
            Gravity::Earth => 4000.0,
            Gravity::Mars => 1517.22731906,
            Gravity::Moon => 660.550458716,
            Gravity::Mercury => 1508.66462793,
            Gravity::Venus => 3616.71763507,
            Gravity::Pluto => 252.803261978,
            Gravity::Sun => 111722.731906,
            _ => 0.0,
        }
    }
}
