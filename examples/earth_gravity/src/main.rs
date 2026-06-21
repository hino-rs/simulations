use macroquad::prelude::*;

const G: f32 = 6.674e-11;
const M: f32 = 5.972e24; // 地球の質量
const R: f32 = 6.371e6;  // 地球の半径
const PIXELS_PER_METER: f32 = 100.0; // 画面上の表示スケール(1メートルを何ピクセルにするか)
const SPEED: f32 = 200.0;
const COR: f32 = 0.9;

struct Coord {
    x: f32,
    y: f32,
}

#[macroquad::main("")]
async fn main() {
    let width = screen_width();
    let height = screen_height();

    let origin = Coord {
        x: width / 2.0,
        y: height / 2.0,
    };

    let mut circle_pos = Coord {
        x: origin.x,
        y: 100.0,
    };

    let mut velocity_y = 0.0f32;
    let mut velocity_x = 0.0f32;
    let radius = 50.0f32;

    loop {
        let dt = get_frame_time(); // 前フレームからの経過秒数
        clear_background(BLACK); // 前フレームをクリア

        if is_key_pressed(KeyCode::Left) {
            velocity_x -= dt * SPEED;
        }

        if is_key_pressed(KeyCode::Right) {
            velocity_x += dt * SPEED;
        }

        if is_key_pressed(KeyCode::Space) {
            velocity_y -= dt * SPEED;
        }

        let g = G * M / (R * R);
        velocity_y += g * dt;

        circle_pos.y += (velocity_y * dt) * PIXELS_PER_METER;
        circle_pos.x += (velocity_x * dt) * PIXELS_PER_METER;

        let wall_left = 0.0 + radius;
        let wall_right = width - radius;
        let ground_y = height - radius;

        if circle_pos.x < wall_left {
            circle_pos.x = wall_left;
            velocity_x = -velocity_x * COR;
        }

        if circle_pos.x > wall_right {
            circle_pos.x = wall_right;
            velocity_x = -velocity_x * COR;
        }

        if circle_pos.y > ground_y {
            circle_pos.y = ground_y;
            velocity_y = -velocity_y * COR;   
            velocity_x *= 0.995;
        }

        draw_circle(circle_pos.x, circle_pos.y, radius, WHITE);

        velocity_x *= 0.998;
        velocity_y *= 0.998;

        next_frame().await; // 忘れると無限ループ
    }
}
