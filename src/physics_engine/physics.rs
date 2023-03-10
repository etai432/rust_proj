use macroquad::prelude::*;

pub trait Shape {
    fn get_color(&self) -> Color;
    fn get_size(&self) -> Vec<f32>;
    fn get_shape(&self) -> &str;
    fn get_location(&self) -> (f32, f32);
    fn get_last_location(&self) -> (f32, f32);
    fn get_velocity(&self) -> (f32, f32);
    fn get_acceleration(&self) -> (f32, f32);
    fn update_position(&mut self, dt: f32);
    fn can_exit_screen(&self) -> bool;
    fn get_x_vec(&self) -> (f32, f32);
    fn get_y_vec(&self) -> (f32, f32);
    fn collision(&mut self, other: (f32, f32, f32));
}

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
}

impl Shape for Circle {
    fn get_color(&self) -> Color {
        return self.color;
    }
    fn get_size(&self) -> Vec<f32> {
        vec![self.radius]
    }
    fn get_shape(&self) -> &str {
        return self.shape.as_str();
    }
    fn get_location(&self) -> (f32, f32) {
        return (self.position_x, self.position_y);
    }
    fn get_last_location(&self) -> (f32, f32) {
        return self.position_old;
    }
    fn get_velocity(&self) -> (f32, f32) {
        return (
            self.position_x - self.position_old.0,
            self.position_y - self.position_old.1,
        );
    }
    fn get_acceleration(&self) -> (f32, f32) {
        return self.acceleration;
    }
    fn update_position(&mut self, dt: f32) {
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
        }
    }
    fn can_exit_screen(&self) -> bool {
        self.exit
    }
    fn get_x_vec(&self) -> (f32, f32) {
        (self.position_x - self.radius, self.position_x + self.radius)
    }
    fn get_y_vec(&self) -> (f32, f32) {
        (self.position_y - self.radius, self.position_y + self.radius)
    }
    fn collision(&mut self, other: (f32, f32, f32)) {
        self.position_x += (self.position_x - other.0) / 2.0;
        self.position_y += (self.position_y - other.1) / 2.0;
    }
}

pub fn is_colliding(shape1: &dyn Shape, shape2: &dyn Shape) -> bool {
    let x_vec_1 = shape1.get_x_vec();
    let y_vec_1 = shape1.get_y_vec();
    let x_vec_2 = shape2.get_x_vec();
    let y_vec_2 = shape2.get_y_vec();
    return ((x_vec_1.0 <= x_vec_2.0 && x_vec_1.1 >= x_vec_2.0
        || x_vec_1.1 >= x_vec_2.1 && x_vec_1.0 <= x_vec_2.1)
        || (x_vec_2.0 <= x_vec_1.0 && x_vec_2.1 >= x_vec_1.0
            || x_vec_2.1 >= x_vec_1.1 && x_vec_2.0 <= x_vec_1.1))
        && ((y_vec_1.0 <= y_vec_2.0 && y_vec_1.1 >= y_vec_2.0
            || y_vec_1.1 >= y_vec_2.1 && y_vec_1.0 <= y_vec_2.1)
            || (y_vec_2.0 <= y_vec_1.0 && y_vec_2.1 >= y_vec_1.0
                || y_vec_2.1 >= y_vec_1.1 && y_vec_2.0 <= y_vec_1.1));
}
