use gdk;
use macroquad::prelude::*;
pub struct Circle {
    pub color: Color,
    pub position_x: f32,
    pub position_y: f32,
    pub velocity: (f32, f32),
    pub acceleration: (f32, f32),
    pub radius: f32,
}

impl Circle {
    pub fn new(
        color: Color,
        position: (f32, f32),
        velocity: (f32, f32),
        gravity: (f32, f32),
        radius: f32,
        exit: bool,
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
    pub fn update_position(&mut self, dt: f32) {
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
