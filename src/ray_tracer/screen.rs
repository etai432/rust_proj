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

fn get_frame(shapes: &Vec<&dyn Volume>, camera: &Camera) -> Image {
    let mut image = Image::gen_image_color(camera.screen.0 as u16, camera.screen.1 as u16, BLACK);
    for x in 0..camera.screen.0 {
        for y in 0..camera.screen.1 {
            let ray = Ray {
                origin: camera.position,
                direction: camera.direction, //make the line go through every pixel
            };
            for i in shapes.iter() {
                let mut min_d = std::f32::MAX;
                let mut color: (u8, u8, u8) = (0, 0, 0);
                match i.get_intersection(&ray) {
                    Some(d) => if d < min_d {
                        min_d = d;
                        color = i.color();
                    },
                    None => (),
                }
                image.set_pixel(x as u32, y as u32, Color{r: color.0 as f32 / 255.0, g: color.1 as f32 / 255.0, b: color.2 as f32 / 255.0, a: 1.0});
            }
        }
    }
    image
}

struct Camera {
    screen: (i32, i32),
    position: (f32, f32, f32),
    direction: (f32, f32, f32),
    fov: f32, //might be replaced with distance
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut shapes: Vec<&dyn Volume> = Vec::new();
    shapes.push(&Sphere {
        color: (255, 255, 255),
        center: (0.0, 0.0, 10.0),
        radius: 10.0,
    });
    let mut camera = Camera {
        screen: (screen_width() as i32, screen_height() as i32),
        position: (0.0, 0.0, 0.0),
        direction: (0.0, 0.0, 1.0),
        fov: 90.0,
    };
    loop {
        let image = get_frame(&shapes, &camera);
        let image_texture = Texture2D::from_image(&image);
        draw_texture(image_texture, 0.0, 0.0, WHITE);
        next_frame().await;
    }
}

pub fn run() {
    main();
}