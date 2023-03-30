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
    pub mass: f64,
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
            color,
            position_x: position.0,
            position_y: position.1,
            velocity,
            acceleration: gravity,
            radius,
            bounciness: 100.0,
            mass: radius * radius,
        }
    }
    pub fn update_position(&mut self, dt: f64) {
        self.position_x += self.velocity.0 * dt;
        self.position_y += self.velocity.1 * dt;
        if self.position_x < self.radius {
            self.position_x = self.radius * 2.0 - self.position_x;
            self.velocity.0 *= -1.0 * (self.bounciness / 100.0).sqrt();
        }
        if self.position_x > screen_width() as f64 - self.radius {
            self.position_x = (screen_width() as f64 - self.radius) * 2.0 - self.position_x;
            self.velocity.0 *= -1.0 * (self.bounciness / 100.0).sqrt();
        }
        if self.position_y < self.radius {
            self.position_y = self.radius * 2.0 - self.position_y;
            self.velocity.1 *= -1.0 * (self.bounciness / 100.0).sqrt();
        }
        if self.position_y > screen_height() as f64 - self.radius {
            self.position_y = (screen_height() as f64 - self.radius) * 2.0 - self.position_y;
            self.velocity.1 *= -1.0 * (self.bounciness / 100.0).sqrt();
        }
        self.velocity.0 += self.acceleration.0 * dt;
        self.velocity.1 += self.acceleration.1 * dt;
    }
}

pub fn collision(circles: &mut [Circle], ind: (usize, usize)) {
    let distance = ((circles[ind.0].position_x - circles[ind.1].position_x).powf(2.0)
        + (circles[ind.0].position_y - circles[ind.1].position_y).powf(2.0))
    .sqrt();
    let radii_sum = circles[ind.0].radius + circles[ind.1].radius;
    if distance > radii_sum {
        return;
    }
    let normal_x = (circles[ind.1].position_x - circles[ind.0].position_x) / distance;
    let normal_y = (circles[ind.1].position_y - circles[ind.0].position_y) / distance;

    let relative_velocity_x = circles[ind.1].velocity.0 - circles[ind.0].velocity.0;
    let relative_velocity_y = circles[ind.1].velocity.1 - circles[ind.0].velocity.1;
    let dot_product = relative_velocity_x * normal_x + relative_velocity_y * normal_y;
    let impulse_magnitude = dot_product * (2.0 * circles[ind.0].mass * circles[ind.1].mass)
        / (circles[ind.0].mass + circles[ind.1].mass);

    circles[ind.0].velocity.0 += impulse_magnitude * normal_x / circles[ind.0].mass
        * (circles[ind.0].bounciness / 100.0).sqrt();
    circles[ind.0].velocity.1 += impulse_magnitude * normal_y / circles[ind.0].mass
        * (circles[ind.0].bounciness / 100.0).sqrt();
    circles[ind.1].velocity.0 -= impulse_magnitude * normal_x / circles[ind.1].mass
        * (circles[ind.1].bounciness / 100.0).sqrt();
    circles[ind.1].velocity.1 -= impulse_magnitude * normal_y / circles[ind.1].mass
        * (circles[ind.1].bounciness / 100.0).sqrt();

    let m_overlap = (circles[ind.0].position_y - circles[ind.1].position_y)
        / (circles[ind.0].position_x - circles[ind.1].position_x);
    let d_overlap = circles[ind.0].radius + circles[ind.1].radius
        - ((circles[ind.0].position_x - circles[ind.1].position_x).powf(2.0)
            + (circles[ind.0].position_y - circles[ind.1].position_y).powf(2.0))
        .sqrt();
    let x = d_overlap / (m_overlap + 1.0);
    let y = x * m_overlap;
    circles[ind.0].position_y += y;
    circles[ind.0].position_x += x;
    circles[ind.1].position_y -= y;
    circles[ind.1].position_x -= x;
}

#[derive(EnumIter, Debug, Clone)]
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
            Gravity::None => 0.0,
        }
    }
}
