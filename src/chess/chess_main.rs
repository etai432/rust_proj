use macroquad::prelude::*;
use image;

fn window_conf() -> Conf {
    Conf {
        window_title: "chess".to_owned(),
        // window_height: screen_height() as i32,
        // window_width: screen_width() as i32,
        fullscreen: true,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let image = Image::empty();
    let board = Texture2D::from_file_with_format(include_bytes!(r"/home/itay/Documents/GitHub/rust_proj/src/chess/board.png"), None);
    draw_texture(board, 0.0, 0.0, WHITE);
    loop{
        next_frame().await;
    }
}

pub fn run() {
    main();
}