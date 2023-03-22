use crate::physics_engine::physics::{collision, is_colliding, Circle, Gravity};
use egui::{Color32, Vec2};
use macroquad::prelude::*;
use std::env;
use strum::IntoEnumIterator;

fn window_conf() -> Conf {
    let abs_path = env::current_dir().unwrap();
    if abs_path.to_str().unwrap().chars().next().unwrap() == '/' {
        return Conf {
            window_title: "physics engine".to_owned(),
            fullscreen: true,
            window_resizable: true,
            ..Default::default()
        };
    }
    return Conf {
        window_title: "physics engine".to_owned(),
        fullscreen: true,
        window_resizable: false,
        ..Default::default()
    };
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

fn change_grav(circles: &mut Vec<Circle>, grav: f64) {
    for circle in circles.iter_mut() {
        circle.acceleration = (0.0, grav);
    }
}

fn change_bounciness(circles: &mut Vec<Circle>, bounciness: f64) {
    for circle in circles.iter_mut() {
        circle.bounciness = bounciness;
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
        (0.0, 4000.0),
        50.0,
    ));
    let mut menu = 0;
    let mut spawn = false;
    let mut speed = (0.0, 0.0);
    let mut size = 1.0;
    let mut color: (u8, u8, u8) = (0, 0, 0);
    let mut bounciness = 100;
    loop {
        let start = get_time();
        clear_background(BLACK);
        if is_key_pressed(KeyCode::Q) {
            break;
        }
        draw(&circles);
        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("menu")
                .fixed_pos(egui::Pos2::new(0.0, 0.0))
                .show(egui_ctx, |ui| {
                    if ui.button("gravity").clicked() {
                        menu = 1;
                    }
                    if ui.button("spawn objects").clicked() {
                        menu = 2;
                    }
                    if ui.button("color selector").clicked() {
                        menu = 3;
                    }
                    if ui.button("bounciness selector").clicked() {
                        menu = 4;
                    }
                });
            if menu == 1 {
                egui::Window::new("gravity")
                    .fixed_pos(egui::Pos2::new(150.0, 0.0))
                    .show(egui_ctx, |ui| {
                        for gravity in Gravity::iter() {
                            if ui.button(format!("{:?}", gravity)).clicked() {
                                change_grav(&mut circles, gravity.get_gravity());
                            }
                        }
                    });
            }
            if menu == 2 {
                spawn = true;
                egui::Window::new("press to spawn")
                    .fixed_pos(egui::Pos2::new(150.0, 0.0))
                    .show(egui_ctx, |ui| {
                        ui.add(egui::Slider::new(&mut size, 1.0..=100.0).text("size"));
                        ui.add(egui::Slider::new(&mut speed.0, -500.0..=500.0).text("speed x"));
                        ui.add(egui::Slider::new(&mut speed.1, -500.0..=500.0).text("speed y"));
                    });
                //implement: press to spawn ballz
            } else {
                spawn = false;
            }
            if menu == 3 {
                egui::Window::new("press to spawn")
                    .fixed_pos(egui::Pos2::new(150.0, 0.0))
                    .show(egui_ctx, |ui| {
                        ui.add(egui::Slider::new(&mut color.0, 0..=255).text("red"));
                        ui.add(egui::Slider::new(&mut color.1, 0..=255).text("green"));
                        ui.add(egui::Slider::new(&mut color.2, 0..=255).text("blue"));
                        egui::Frame::none()
                            .fill(Color32::from_rgb(color.0, color.1, color.2))
                            .show(ui, |ui| {
                                ui.allocate_exact_size(
                                    Vec2::new(100.0, 20.0),
                                    egui::Sense::click(),
                                );
                            });
                    });
            }
            if menu == 4 {
                egui::Window::new("bounciness picker")
                    .fixed_pos(egui::Pos2::new(150.0, 0.0))
                    .default_size(Vec2::new(200.0, 200.0))
                    .show(egui_ctx, |ui| {
                        let temp = bounciness;
                        ui.add(egui::Slider::new(&mut bounciness, 75..=100).text("bounciness %"));
                        if temp != bounciness {
                            change_bounciness(&mut circles, bounciness as f64)
                        }
                    });
            }
        });
        egui_macroquad::draw();
        // println!("{}", get_fps());
        next_frame().await;
        update(&mut circles, get_time() - start);
    }
}

pub fn run() {
    main();
}

//TODO:
//spawn balls by clicking
//collisions
