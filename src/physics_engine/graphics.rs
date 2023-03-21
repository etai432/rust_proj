use crate::physics_engine::physics::{collision, is_colliding, Circle};
use egui::{Button, Slider};
use macroquad::prelude::*;

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
            circle.position_x as f32,
            circle.position_y as f32,
            circle.radius as f32,
            circle.color,
        )
    }
}

fn update(shapes: &mut Vec<Circle>, dt: f64) {
    for circle in shapes.iter_mut() {
        circle.update_position(dt);
    }
}

fn collisions(circles: &mut Vec<Circle>) {}

#[macroquad::main(window_conf)]
async fn main() {
    let mut circles: Vec<Circle> = Vec::new();
    circles.push(Circle::new(
        WHITE,
        (500.0, 200.0),
        (500.0, 0.0),
        (0.0, 0.98),
        50.0,
    ));
    loop {
        let start = get_time();
        clear_background(BLACK);
        if is_key_pressed(KeyCode::Q) {
            break;
        }
        draw(&circles);
        let mut value = 0;
        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("menu")
                .fixed_pos(egui::Pos2::new(0.0, 0.0))
                .show(egui_ctx, |ui| {
                    // ui.vertical(|ui| {
                    //     if ui.button("Click each year").clicked() {
                    //         println!("clicked");
                    //     }
                    //     ui.add(Button::new("hi"));
                    //     ui.add(Button::new("hi"));
                    // });
                    if ui.button("gravity").clicked() {
                        println!("clicked");
                    }
                    ui.add(Button::new("hi"));
                    ui.add(Button::new("hi"));
                });
        });
        egui_macroquad::draw();
        // println!("{:?}", circles[0].velocity.1);
        // println!("{}", is_colliding(&circles[0], &circles[1]));
        // collisions(&mut circles);
        // sleep(time::Duration::from_secs(1));
        // println!("{}", get_fps());
        next_frame().await;
        update(&mut circles, get_time() - start);
    }
}

pub fn run() {
    main();
}
