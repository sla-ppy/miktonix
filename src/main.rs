mod prefix;
use macroquad::prelude::*;
use std::io;
use std::io::*;

struct Component {
    voltage: f64,
    current: f64,
    resistance: f64,
    // rectangle params
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

fn draw_menu() {
    draw_text("Create component", screen_width() / 1.45, 32.0, 32.0, BLACK);
    draw_text("Remove component", screen_width() / 1.45, 64.0, 32.0, BLACK);
    draw_text("Move component", screen_width() / 1.45, 96.0, 32.0, BLACK);
}

impl Component {
    fn new(voltage: f64, current: f64, resistance: f64, x: f32, y: f32, w: f32, h: f32) -> Component {
        Component { voltage, current, resistance, x, y, w, h }
    }
}

#[macroquad::main("Resistance Calculator")]
async fn main() {
    let component1 = Component::new(15.0, 2.0, 2.0, screen_width() / 10.0 - 60.0, 100.0, 60.0, 30.0);
    let offset = 100.0; // new part offset

    loop {
        clear_background(LIGHTGRAY);

        draw_menu();

        draw_rectangle(component1.x, component1.y, component1.w, component1.h, BLACK);
        draw_text("R1", component1.x + (component1.w / 3.0), component1.y + (component1.h / 1.5), 20.0, WHITE);

        //draw_rectangle(posx + offset, posy, width, height, BLACK);
        //draw_line(posx + width, posy + (height / 2.0), posx + offset , posy + (height / 2.0), 2.5 ,BLACK);
        //draw_text("R2", posx + offset + (width / 3.0), posy + (height / 1.5), 20.0, WHITE);

        if is_key_down(KeyCode::Escape) {
            return
        }

        next_frame().await
    }
}