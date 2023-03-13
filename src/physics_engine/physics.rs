use ::rand::seq::index;
use macroquad::prelude::*;
pub struct Circle {
    pub color: Color,
    pub position_x: f32,
    pub position_y: f32,
    pub position_old: (f32, f32),
    pub acceleration: (f32, f32),
    pub shape: String,
    pub radius: f32,
    pub exit: bool,
}

impl Circle {
    pub fn new(
        color: Color,
        position: (f32, f32),
        speed: (f32, f32),
        gravity: (f32, f32),
        radius: f32,
        exit: bool,
    ) -> Self {
        Circle {
            color: color,
            position_x: position.0,
            position_y: position.1,
            position_old: (position.0 - speed.0, position.1 - speed.1),
            acceleration: gravity,
            shape: "Circle".to_string(),
            radius: radius,
            exit: exit,
        }
    }
    pub fn get_location(&self) -> (f32, f32) {
        return (self.position_x, self.position_y);
    }
    fn get_velocity(&self) -> (f32, f32) {
        return (
            self.position_x - self.position_old.0,
            self.position_y - self.position_old.1,
        );
    }
    pub fn update_position(&mut self, dt: f32) {
        let velocity = self.get_velocity();
        self.position_old = (self.position_x, self.position_y);
        self.position_x += velocity.0 + self.acceleration.0 * dt * dt;
        self.position_y += velocity.1 + self.acceleration.1 * dt * dt;
        if !self.exit {
            if self.position_x < self.radius {
                let temp = self.position_old;
                self.position_old = (self.position_x, self.position_old.1);
                self.position_x = temp.0;
            }
            if self.position_x > screen_width() - self.radius {
                let temp = self.position_old;
                self.position_old = (self.position_x, self.position_old.1);
                self.position_x = temp.0;
            }
            if self.position_y > screen_height() - self.radius {
                let temp = self.position_old;
                self.position_old = (self.position_old.0, self.position_y);
                self.position_y = temp.1;
            }
            if self.position_y < self.radius {
                let temp = self.position_old;
                self.position_old = (self.position_old.0, self.position_y);
                self.position_y = temp.1;
            }
        }
    }
}

pub fn is_colliding(circle1: &Circle, circle2: &Circle) -> bool {
    ((circle1.position_x - circle2.position_x).powf(2.0)
        + (circle1.position_y - circle2.position_y).powf(2.0))
    .sqrt()
        < circle1.radius + circle2.radius
}

pub fn collision(circles: &mut Vec<Circle>, indexes: (usize, usize)) {
    let m_overlap = (circles[indexes.0].position_y - circles[indexes.1].position_y)
        / (circles[indexes.0].position_x - circles[indexes.1].position_x);
    let d_overlap = circles[indexes.0].radius + circles[indexes.1].radius
        - ((circles[indexes.0].position_x - circles[indexes.1].position_x).powf(2.0)
            + (circles[indexes.0].position_y - circles[indexes.1].position_y).powf(2.0))
        .sqrt();
    let x = d_overlap / (m_overlap + 1.0);
    let y = x * m_overlap;
    println!("{}, {}", x, y);
    circles[indexes.0].position_y += y;
    circles[indexes.0].position_x += x;
    circles[indexes.1].position_y -= y;
    circles[indexes.1].position_x -= x;
}
