use ::rand::prelude::*;
use macroquad::prelude::*;
// use std::time::{Duration, Instant};

const WIDTH:Option<i32> = Some(800);
const HEIGHT:Option<i32> = Some(600);
const DENSITY: f32 = 0.3;
// const BLACK1: Color = Color {r: 0.0, g: 0.0, b: 0.0, a: 0.0};
const GREEN1: Color = Color {r: 0.0, g: 0.8862745, b: 0.1882353, a: 1.0};

fn window_conf() -> Conf {
    match HEIGHT {
        Some(_) => Conf {
            window_title: "Conway's game of life".to_owned(),
            window_height: HEIGHT.unwrap(),
            window_width: WIDTH.unwrap(),
            window_resizable: false,
            ..Default::default()
        },
        None => Conf {
            window_title: "Conway's game of life".to_owned(),
            fullscreen: true,
            window_resizable: false,
            ..Default::default()
        }
    }
}

fn randomize(density: f32, mut image: Image) -> Image {
    let mut rng = thread_rng();
    for x in 0..image.width as u32 {
        for y in 0..image.height as u32 {
            if density >= rng.gen_range(0.0..1.0) {
                image.set_pixel(x, y, GREEN)
            }
        }
    }
    return image;
}

fn udpate_living_cells(mut image: Image) -> Image {
    let im = image.clone();
    let mut live_neighbors: i32;
    for x in 0..im.width as u32 {
        for y in 0..im.height as u32 {
            live_neighbors = 0;
            if x != 0 {
                if im.get_pixel(x - 1, y) == GREEN1 {
                    live_neighbors += 1;
                }
                if y != 0 {
                    if im.get_pixel(x - 1, y - 1) == GREEN1 {
                        live_neighbors += 1;
                    }
                }
                if y != im.height as u32 - 1 {
                    if im.get_pixel(x - 1, y + 1) == GREEN1 {
                        live_neighbors += 1;
                    }
                }
            }
            if y != 0 {
                if im.get_pixel(x, y - 1) == GREEN1 {
                    live_neighbors += 1;
                }
                if x != im.width as u32 - 1 {
                    if image.get_pixel(x + 1, y - 1) == GREEN1 {
                        live_neighbors += 1;
                    }
                }
            }
            if y != im.height as u32 - 1 {
                if im.get_pixel(x, y + 1) == GREEN1 {
                    live_neighbors += 1;
                }
                if x != im.width as u32 - 1 {
                    if im.get_pixel(x + 1, y + 1) == GREEN1 {
                        live_neighbors += 1;
                    }
                }
            }
            if x != im.width as u32 - 1 {
                if im.get_pixel(x + 1, y) == GREEN1 {
                    live_neighbors += 1;
                }
            }
            if im.get_pixel(x, y) == GREEN1 {
                if live_neighbors != 2 && live_neighbors != 3 {
                    image.set_pixel(x, y, BLACK);
                }
            }
            else {
                if live_neighbors == 3 {
                    image.set_pixel(x, y, GREEN);
                }
            }
        }
    }
    return image;
}

fn is_alive(image: &Image) -> bool {
    for x in 0..image.width as u32 {
        for y in 0..image.height as u32 {
            if image.get_pixel(x, y) == GREEN1 {
                return true;
            }
        }
    }
    return false;
}

fn population(image: &Image) -> i32 {
    let mut counter: i32 = 0;
    for x in 0..image.width as u32 {
        for y in 0..image.height as u32 {
            if image.get_pixel(x, y) == GREEN1 {
                counter += 1;
            }
        }
    }
    return counter;
}
#[macroquad::main(window_conf)]
async fn main() {
    let mut image = Image::gen_image_color(WIDTH.unwrap_or(screen_width() as i32) as u16, HEIGHT.unwrap_or(screen_height() as i32) as u16, BLACK);
    image = randomize(DENSITY, image);
    let mut pop: i32;
    let mut gen: i32 = 0;
    while is_alive(&image) {
        pop = population(&image);
        image = udpate_living_cells(image);
        gen += 1;
        let image_texture = Texture2D::from_image(&image);
        draw_texture(image_texture, 0.0, 0.0, WHITE);
        draw_text(format!("population: {}", pop).as_str(), 0.0, 26.0, 50.0, WHITE);
        draw_text(format!("generation: {}", gen).as_str(), 0.0, 51.0, 40.0, WHITE);
        next_frame().await;
    }
}

pub fn run() {
    main();
    // let start = Instant::now();
    // let duration = start.elapsed();
    // println!("Time elapsed is: {:?}", duration);
}