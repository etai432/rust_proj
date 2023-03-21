use macroquad::prelude::*;
use winit::dpi;
pub struct Circle {
    pub color: Color,
    pub position_x: f64,
    pub position_y: f64,
    pub velocity: (f64, f64),
    pub acceleration: (f64, f64),
    pub radius: f64,
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
        }
    }
    pub fn update_position(&mut self, dt: f64) {
        self.position_x += self.velocity.0 * dt + 0.5 * self.acceleration.0 * dt * dt;
        self.position_y += self.velocity.1 * dt + 0.5 * self.acceleration.1 * dt * dt;
        if self.position_x < self.radius {
            self.position_x = self.radius * 2.0 - self.position_x;
            self.velocity.0 *= -1.0;
        }
        if self.position_x > screen_width() as f64 - self.radius {
            self.position_x = (screen_width() as f64 - self.radius) * 2.0 - self.position_x;
            self.velocity.0 *= -1.0;
        }
        if self.position_y < self.radius {
            self.position_y = self.radius * 2.0 - self.position_y;
            self.velocity.1 *= -1.0;
        }
        if self.position_y > screen_height() as f64 - self.radius {
            self.position_y = (screen_height() as f64 - self.radius) * 2.0 - self.position_y;
            self.velocity.1 *= -1.0;
        }
        self.velocity.0 += self.acceleration.0;
        self.velocity.1 += self.acceleration.1;
    }
}

pub fn is_colliding(circle1: &Circle, circle2: &Circle) -> bool {
    ((circle1.position_x - circle2.position_x).powf(2.0)
        + (circle1.position_y - circle2.position_y).powf(2.0))
    .sqrt()
        < circle1.radius + circle2.radius
}

pub fn collision(circles: &mut Vec<Circle>, indexes: (usize, usize)) {}

pub enum Gravity {
    Earth,
    Mars,
    Moon,
    Mercury,
    Venus,
    Pluto,
}

impl Gravity {
    pub fn get_gravity(self) -> f64 {
        //find screen size
        match self {
            Earth => (),
            Mars => (),
            Moon => (),
            Mercury => (),
            Venus => (),
            Pluto => (),
        }
        return 0.0;
    }
}
