use crate::ray_tracer::shapes::*;
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Ray tracer".to_owned(),
        fullscreen: true,
        window_resizable: false,
        ..Default::default()
    }
}

fn get_frame(shapes: Vec<&dyn Volume>) -> Image {

}

struct Camera {
    screen: (i32, i32),
    screen_pos: (f32, f32, f32), // the middle of the screen
    position: (f32, f32, f32),
    direction: (f32, f32, f32),
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut shapes: Vec<&dyn Volume> = Vec::new();
    loop {
        next_frame().await;
    }
}

pub fn run() {
    main();
}