use crate::physics_engine::physics::{collision, is_colliding, Circle};
use core::time;
use macroquad::prelude::*;
use std::thread::sleep;

fn window_conf() -> Conf {
    Conf {
        window_title: "physics engine".to_owned(),
        fullscreen: true,
        window_resizable: false,
        ..Default::default()
    }
}

fn draw(circles: &Vec<Circle>) {
    for circle in circles.into_iter() {
        draw_circle(
            circle.position_x,
            circle.position_y,
            circle.radius,
            circle.color,
        )
    }
}

fn update(shapes: &mut Vec<Circle>, dt: f32) {
    for circle in shapes.iter_mut() {
        circle.update_position(dt);
    }
}

fn collisions(circles: &mut Vec<Circle>) {
    // for substeps in 0..substeps {
    for i in 0..circles.len() {
        for j in 0..circles.len() {
            if i != j && is_colliding(&circles[i], &circles[j]) {
                collision(circles, (i, j));
            }
        }
    }
    // }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut circles: Vec<Circle> = Vec::new();
    circles.push(Circle::new(
        WHITE,
        (500.0, 200.0),
        (0.1, 0.0),
        (0.0, 1.0),
        50.0,
        false,
    ));
    loop {
        if is_key_pressed(KeyCode::Q) {
            break;
        }
        draw(&circles);
        update(&mut circles, 0.01);
        // println!("{:?}", circles[0].get_location());
        // println!("{}", is_colliding(&circles[0], &circles[1]));
        collisions(&mut circles);
        // sleep(time::Duration::from_secs(1));
        // println!("{}", get_fps());
        next_frame().await;
    }
}

pub fn run() {
    main();
}
