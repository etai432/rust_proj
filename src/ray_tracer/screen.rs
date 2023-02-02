use crate::ray_tracer::shapes::*;
use macroquad::prelude::*;
use std::f32;

fn window_conf() -> Conf {
    Conf {
        window_title: "Ray tracer".to_owned(),
        fullscreen: true,
        window_resizable: false,
        ..Default::default()
    }
}

fn normalize(v: (f32, f32, f32)) -> (f32, f32, f32) {
    let len = (v.0 * v.0 + v.1 * v.1 + v.2 * v.2).sqrt();
    (v.0 / len, v.1 / len, v.2 / len)
}

fn angle_between_ray_and_normal(ray: &Ray, normal: &(f32, f32, f32)) -> f32 {
    let dot_product = ray.direction.0 * normal.0 + ray.direction.1 * normal.1 + ray.direction.2 * normal.2;
    let ray_length = (ray.direction.0 * ray.direction.0 + ray.direction.1 * ray.direction.1 + ray.direction.2 * ray.direction.2).sqrt();
    let normal_length = (normal.0 * normal.0 + normal.1 * normal.1 + normal.2 * normal.2).sqrt();
    let cosine = dot_product / (ray_length * normal_length);
    cosine.acos()
}

fn change_rgb_color(rgb: &(u8, u8, u8), angle: f32) -> (u8, u8, u8) {
    let intensity = (angle / f32::consts::PI).powf(2.0);
    (
        (rgb.0 as f32 * intensity) as u8,
        (rgb.1 as f32 * intensity) as u8,
        (rgb.2 as f32 * intensity) as u8,
    )
}

fn get_frame(shapes: &Vec<&dyn Render>, camera: &Camera, skip: usize) -> Image {
    let mut image = Image::gen_image_color(camera.screen.0 as u16, camera.screen.1 as u16, BLACK);
    for x in (0..camera.screen.0).step_by(skip) {
        for y in (0..camera.screen.1).step_by(skip) {
            let ray = Ray {
                origin: camera.position,
                direction: normalize(
                    (
                      (x as f32 - camera.screen.0 as f32 / 2.0) / (camera.screen.0 as f32 / 2.0) * (camera.screen.0 as f32 / camera.screen.1 as f32) + camera.direction.0,
                      -(y as f32 - camera.screen.1 as f32 / 2.0) / (camera.screen.1 as f32 / 2.0) + camera.direction.1,
                      camera.direction.2
                    ))
            };
            let mut min_d = std::f32::MAX;
            let mut color: (u8, u8, u8) = (0, 0, 0);
            for i in shapes.iter() {
                match i.get_intersection(&ray) {
                    Some(d) => if d.0 < min_d {
                        min_d = d.0;
                        color = i.color();
                        let angle = angle_between_ray_and_normal(&ray, &d.1);
                        color = change_rgb_color(&color, angle);
                    },
                    None => (),
                }
                for x1 in x..(x+skip as i32) {
                    for y1 in y..(y+skip as i32) {
                        image.set_pixel(x1 as u32, y1 as u32, Color{r: color.0 as f32/ 255.0, g: color.1 as f32 / 255.0, b: color.2 as f32 / 255.0, a: 1.0});
                    }
                }
            }
        }
    }
    return image;
}

struct Camera {
    screen: (i32, i32),
    position: (f32, f32, f32),
    direction: (f32, f32, f32),
    fov: f32,
}


#[macroquad::main(window_conf)]
async fn main() {
    let mut shapes: Vec<&dyn Render> = Vec::new();
    shapes.push(&Sphere {
        color: (0, 0, 255),
        center: (0.0, 0.0, 500.0),
        radius: 200.0,
    });
    shapes.push(&Sphere {
        color: (255, 0, 0),
        center: (100.0, 0.0, 300.0),
        radius: 100.0,
    });
    // shapes.push(&Sphere {
    //     color: (0, 0, 255),
    //     center: (-120.0, 100.0, -350.0),
    //     radius: 100.0,
    // });
    let mut camera = Camera {
        screen: (screen_width() as i32, screen_height() as i32),
        position: (100.0, 0.0, 150.0),
        direction: (0.0, 0.0, -1.0),
        fov: 90.0,
    };
    let mut count = 0;
    loop {
        count += 1;
        let image = get_frame(&shapes, &camera, 1);
        let image_texture = Texture2D::from_image(&image);
        draw_texture(image_texture, 0.0, 0.0, WHITE);
        next_frame().await;
        println!("{}", count);
    }
}

pub fn run() {
    main();
}