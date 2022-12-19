use macroquad::prelude::*;
use image::*;
mod chess_utils;
use self::chess_utils::*;


fn window_conf() -> Conf {
    Conf {
        window_title: "chess".to_owned(),
        window_height: 800,
        window_width: 800,
        window_resizable: false,
        ..Default::default()
    }
}

fn copy_image(image: &DynamicImage) -> Image {
    let dim = image.dimensions();
    let mut image1 = Image::gen_image_color(dim.0 as u16, dim.1 as u16, GREEN);
    for x in 0..dim.0 {
        for y in 0..dim.1 {
            let pixel = image.get_pixel(x, y);
            image1.set_pixel(x, y, Color { r: pixel.0[0] as f32 / 255.0, g: pixel.0[1] as f32 / 255.0, b: pixel.0[2] as f32 / 255.0, a: pixel.0[3] as f32 / 255.0})
        }
    }
    return image1;
}
//board array, board, king-1, queen-2, rook-3, bishop-4, horse-5, pawn-6. negative for black
fn restart() -> (Vec<i32>, Texture2D, Texture2D, Texture2D, Texture2D, Texture2D, Texture2D, Texture2D, Texture2D, Texture2D, Texture2D, Texture2D, Texture2D, Texture2D) {
    let board_arr = vec![-3, -5, -4, -2, -1, -4, -5, -3, -6, -6, -6, -6, -6, -6 ,-6 ,-6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 6, 6, 6, 6, 6, 6, 6, 3, 5, 4, 2, 1, 4, 5, 3];
    // let board_arr = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut img1 = image::open("src/chess/board.png").unwrap();
    img1 = img1.resize(800, 800, image::imageops::FilterType::Gaussian);
    let board = Texture2D::from_image(&copy_image(&img1));
    img1 = image::open("src/chess/white_king.png").unwrap();
    img1 = img1.resize(100, 100, imageops::FilterType::Gaussian);
    let wking = Texture2D::from_image(&copy_image(&img1));
    img1 = image::open("src/chess/white_queen.png").unwrap();
    img1 = img1.resize(100, 100, imageops::FilterType::Gaussian);
    let wqueen = Texture2D::from_image(&copy_image(&img1));
    img1 = image::open("src/chess/white_rook.png").unwrap();
    img1 = img1.resize(100, 100, imageops::FilterType::Gaussian);
    let wrook = Texture2D::from_image(&copy_image(&img1));
    img1 = image::open("src/chess/white_bishop.png").unwrap();
    img1 = img1.resize(100, 100, imageops::FilterType::Gaussian);
    let wbishop = Texture2D::from_image(&copy_image(&img1));
    img1 = image::open("src/chess/white_horse.png").unwrap();
    img1 = img1.resize(100, 100, imageops::FilterType::Gaussian);
    let wknight = Texture2D::from_image(&copy_image(&img1));
    img1 = image::open("src/chess/white_pawn.png").unwrap();
    img1 = img1.resize(100, 100, imageops::FilterType::Gaussian);
    let wpawn = Texture2D::from_image(&copy_image(&img1));
    img1 = image::open("src/chess/black_king.png").unwrap();
    img1 = img1.resize(100, 100, imageops::FilterType::Gaussian);
    let bking = Texture2D::from_image(&copy_image(&img1));
    img1 = image::open("src/chess/black_queen.png").unwrap();
    img1 = img1.resize(100, 100, imageops::FilterType::Gaussian);
    let bqueen = Texture2D::from_image(&copy_image(&img1));
    img1 = image::open("src/chess/black_rook.png").unwrap();
    img1 = img1.resize(100, 100, imageops::FilterType::Gaussian);
    let brook = Texture2D::from_image(&copy_image(&img1));
    img1 = image::open("src/chess/black_bishop.png").unwrap();
    img1 = img1.resize(100, 100, imageops::FilterType::Gaussian);
    let bbishop = Texture2D::from_image(&copy_image(&img1));
    img1 = image::open("src/chess/black_horse.png").unwrap();
    img1 = img1.resize(100, 100, imageops::FilterType::Gaussian);
    let bknight = Texture2D::from_image(&copy_image(&img1));
    img1 = image::open("src/chess/black_pawn.png").unwrap();
    img1 = img1.resize(100, 100, imageops::FilterType::Gaussian);
    let bpawn = Texture2D::from_image(&copy_image(&img1));
    return (board_arr, board, wking, wqueen, wrook, wbishop, wknight, wpawn, bking, bqueen, brook, bbishop, bknight, bpawn);
}

fn draw_board(board_arr: &Vec<i32>, board: Texture2D, wking: Texture2D, wqueen: Texture2D, wrook: Texture2D, wbishop: Texture2D, wknight: Texture2D, wpawn: Texture2D, bking: Texture2D, bqueen: Texture2D, brook: Texture2D, bbishop: Texture2D, bknight: Texture2D, bpawn: Texture2D) {
    draw_texture(board, 0.0, 0.0, WHITE);
    for i in 0..board_arr.len() {
        match board_arr[i] {
            1 => draw_texture(wking, (i % 8 * 100) as f32, (i / 8 * 100) as f32, WHITE),
            2 => draw_texture(wqueen, (i % 8 * 100) as f32, (i / 8 * 100) as f32, WHITE),
            3 => draw_texture(wrook, (i % 8 * 100) as f32, (i / 8 * 100) as f32, WHITE),
            4 => draw_texture(wbishop, (i % 8 * 100) as f32, (i / 8 * 100) as f32, WHITE),
            5 => draw_texture(wknight, (i % 8 * 100) as f32, (i / 8 * 100) as f32, WHITE),
            6 => draw_texture(wpawn, (i % 8 * 100) as f32, (i / 8 * 100) as f32, WHITE),
            -1 => draw_texture(bking, (i % 8 * 100) as f32, (i / 8 * 100) as f32, WHITE),
            -2 => draw_texture(bqueen, (i % 8 * 100) as f32, (i / 8 * 100) as f32, WHITE),
            -3 => draw_texture(brook, (i % 8 * 100) as f32, (i / 8 * 100) as f32, WHITE),
            -4 => draw_texture(bbishop, (i % 8 * 100) as f32, (i / 8 * 100) as f32, WHITE),
            -5 => draw_texture(bknight, (i % 8 * 100) as f32, (i / 8 * 100) as f32, WHITE),
            -6 => draw_texture(bpawn, (i % 8 * 100) as f32, (i / 8 * 100) as f32, WHITE),
            _ => (),
        }
    }
}

fn draw_move(index: Vec<usize>, board_arr: &Vec<i32>) {
    for i in index {
        if board_arr[i] == 0 {
            draw_circle(50.0 + (i % 8 * 100) as f32, 50.0 + (i / 8 * 100) as f32, 20.0, Color{r: 0.4, g: 0.4, b: 0.4, a: 0.5});
        }
        else{
            draw_circle_lines(50.0 + (i % 8 * 100) as f32, 50.0 + (i / 8 * 100) as f32, 49.0, 7.0, Color{r: 0.4, g: 0.4, b: 0.4, a: 0.5});
        }
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // let mut img: RgbaImage = ImageBuffer::new(800, 800);
    let (board_arr, board, wking, wqueen, wrook, wbishop, wknight, wpawn, bking, bqueen, brook, bbishop, bknight, bpawn) = restart();
    loop{
        draw_board(&board_arr, board, wking, wqueen, wrook, wbishop, wknight, wpawn, bking, bqueen, brook, bbishop, bknight, bpawn);
        draw_move(gen_moves_pawn(9, &board_arr, false, &board_arr), &board_arr);
        next_frame().await;
    }
}

pub fn run() {
    main();
    todo!("implement castling, en passant, pawns becoming queens, stalemate, repetition, ");
    todo!("input function");
    todo!("player_turn function");
    todo!("2_players function");
    todo!("evaluation function- possibly using a neural network");
    todo!("minimax");
    todo!("alpha-beta pruning");
    todo!("player vs computer function");
    todo!("computer vs computer function");
}