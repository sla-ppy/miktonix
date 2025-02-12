mod prefix;
use macroquad::prelude::*;
use std::io;
use std::io::*;

struct Part {
    voltage: f64,
    current: f64,
    resistance: f64,

    // rectangle params
    posx: f32,
    posy: f32,
    width: f32,
    height: f32,
}

fn create_component() {}

fn draw_menu() {
    draw_text("Create component", screen_width() / 1.45, 32.0, 32.0, BLACK);
    draw_text("Remove component", screen_width() / 1.45, 64.0, 32.0, BLACK);
    draw_text("Move component", screen_width() / 1.45, 96.0, 32.0, BLACK);
}

#[macroquad::main("Resistance Calculator")]
async fn main() {
    //create_component();

    let posx = screen_width() / 10.0 - 60.0;
    let posy = 100.0;
    let width = 60.0;
    let height = 30.0;
    let offset = 100.0; // new part offset

    loop {
        clear_background(LIGHTGRAY);

        draw_menu();

        draw_rectangle(posx, posy, width, height, BLACK);
        draw_text("R1", posx + (width / 3.0), posy + (height / 1.5), 20.0, WHITE);


        draw_rectangle(posx + offset, posy, width, height, BLACK);
        draw_line(posx + width, posy + (height / 2.0), posx + offset , posy + (height / 2.0), 2.5 ,BLACK);
        draw_text("R2", posx + offset + (width / 3.0), posy + (height / 1.5), 20.0, WHITE);

        if is_key_down(KeyCode::Escape) {
            return
        }

        next_frame().await
    }
}