use crate::physics_engine::physics::{is_colliding, Circle, Shape};
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

fn draw(shapes: &Vec<Box<dyn Shape>>) {
    for shape in shapes.into_iter() {
        match shape.get_shape() {
            "Circle" => draw_circle(
                shape.get_location().0,
                shape.get_location().1,
                shape.get_size()[0],
                shape.get_color(),
            ),
            _ => (),
        }
    }
}

fn update(shapes: &mut Vec<Box<dyn Shape>>, dt: f32) {
    for shape in shapes.iter_mut() {
        shape.update_position(dt);
    }
}

fn collisions(shapes: &mut Vec<Box<dyn Shape>>, substeps: i32) {
    let loc = shapes
        .iter()
        .map(|shape| {
            (
                shape.get_location().0,
                shape.get_location().1,
                shape.get_size()[0],
            )
        })
        .collect::<Vec<(f32, f32, f32)>>();
    // for _ in 0..substeps {
    for i in 0..shapes.len() {
        for j in 0..shapes.len() {
            if j != i && is_colliding(shapes[i].as_ref(), shapes[j].as_ref()) {
                shapes[i].collision(loc[j]);
            }
        }
    }
    // }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut shapes: Vec<Box<dyn Shape>> = Vec::new();
    shapes.push(Box::from(Circle::new(
        WHITE,
        (500.0, 200.0),
        (0.1, 0.0),
        (0.0, 1.0),
        50.0,
        false,
    )));
    shapes.push(Box::from(Circle::new(
        RED,
        (300.0, 200.0),
        (0.0, 0.0),
        (0.0, 0.0),
        100.0,
        false,
    )));
    loop {
        if is_key_pressed(KeyCode::Q) {
            break;
        }
        clear_background(BLACK);
        draw(&shapes);
        update(&mut shapes, 0.01);
        // println!("{:?}", shapes[0].get_location());
        // println!("{}", is_colliding(shapes[0].as_ref(), shapes[1].as_ref()));
        collisions(&mut shapes, 8);
        // sleep(time::Duration::from_secs(1));
        next_frame().await;
    }
}

pub fn run() {
    main();
}
