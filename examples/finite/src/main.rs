use std::io::Write;

use macroquad::prelude::*;

const G: f64 = 6.674e-11;
// const EARTH_M: f64 = 5.972e24;
const EARTH_M: f64 = 5.972e30;
// const EARTH_M: f64 = 8.54e36;
const EARTH_R: f64 = 6.371e6;
const PIXELS_PER_METER: f64 = 1.0e-5;
const SIMULATION_SPEED: f64 = 100.0;

struct Spacecraft {
    pos: DVec2,
    angle: f64,
    jet_vel: f64,
    vel: DVec2,    
    mass: f64,
}

struct Planet {
    pos: DVec2,
    mass: f64,
    r: f64,
}

impl Spacecraft {
    fn draw(&self, x: f64, y: f64, h: f64, w: f64) {
        let cx = x as f32;
        let cy = y as f32;
        let angle = self.angle as f32;

        let hw = (w / 2.0) as f32;
        let hh = (h / 2.0) as f32;
        let cos = angle.cos();
        let sin = angle.sin();

        let vertices = [
            vec2(-hw, -hh),
            vec2( hw, -hh),
            vec2( hw,  hh),
            vec2(-hw,  hh),
        ]
        .map(|v| vec2(
            cx + v.x * cos - v.y * sin,
            cy + v.x * sin + v.y * cos,
        ));

        draw_triangle(vertices[0], vertices[1], vertices[2], WHITE);
        draw_triangle(vertices[0], vertices[2], vertices[3], WHITE);
    }

    fn jet(&mut self, dt: f64) {
        let delta_x = self.jet_vel * self.angle.cos();
        let delta_y = self.jet_vel * self.angle.sin();
        
        self.vel.x += delta_x * dt;
        self.vel.y += delta_y * dt;
    }

    fn stop(&mut self) {
        self.vel = DVec2::ZERO;
    }
}

fn linear_motion(machine: &mut Spacecraft, dt: f64) {
    // let delta_x = machine.vel.x * machine.angle.cos();
    // let delta_y = machine.vel.y * machine.angle.sin();

    machine.pos.x += machine.vel.x * dt * PIXELS_PER_METER;
    machine.pos.y += machine.vel.y * dt * PIXELS_PER_METER;
}

#[macroquad::main("")]
async fn main() {
    let width = screen_width() as f64;
    let height = screen_height() as f64;
    let center = DVec2::new(width / 2.0, height / 2.0);

    let earth = Planet {
        pos: DVec2::ZERO,
        mass: EARTH_M,
        r: EARTH_R,
    };
    let draw_earth_x = center.x + earth.pos.x * PIXELS_PER_METER;
    let draw_earth_y = center.y + earth.pos.y * PIXELS_PER_METER;

    let mut machine = Spacecraft {
        pos: DVec2 { 
            // x: -(EARTH_R + 400e3),
            x: -(EARTH_R + 16_000_000.0),
            y: 0.0,
        },
        angle: 0.0,
        jet_vel: 100.0,
        vel: DVec2::ZERO,
        mass: 2800.0 * 1000.0,
    };

    loop {
        let dt = get_frame_time() as f64 * SIMULATION_SPEED; // 前フレームからの経過秒数
        clear_background(BLACK); // 前フレームをクリア

        // draw_text(format!("Jet Velocity: {}\nVelocity X:{}, Y:{}\n Position X:{}, Y:{}", machine.jet_vel, machine.vel.x, machine.vel.y, machine.pos.x, machine.pos.y), width - 200.0, 20.0, 32.0, WHITE);
        
        let position_text = format!("Position: ({:.1}km, {:.1}km)", machine.pos.x, machine.pos.y);
        let position_text_dims = measure_text(&position_text, None, 32, 1.0);
        draw_text(
            &position_text, 
            screen_width() * 0.9 - position_text_dims.width / 2.0, 
            screen_height() * 0.9, 
            32.0, 
            WHITE
        );

        let draw_machine_x = center.x + machine.pos.x * PIXELS_PER_METER;
        let draw_machine_y = center.y + machine.pos.y * PIXELS_PER_METER;
        machine.draw(draw_machine_x, draw_machine_y, 10.0, 30.0);

        if is_key_pressed(KeyCode::E) {
            machine.jet_vel *= 2.0;
        }
        if is_key_pressed(KeyCode::Q) {
            machine.jet_vel /= 2.0;
        }

        if is_key_down(KeyCode::Left) {
            machine.angle -= 3.0 * (dt / SIMULATION_SPEED);
        }
        if is_key_down(KeyCode::Right) {
            machine.angle += 3.0 * (dt / SIMULATION_SPEED);
        }

        if is_key_down(KeyCode::Space) {
            machine.jet(dt);
        }
        if is_key_pressed(KeyCode::LeftControl) {
            machine.stop();
        }

        linear_motion(&mut machine, dt);

        let dist_m = earth.pos - machine.pos;
        let r = dist_m.length();
        let dir = dist_m.normalize();
        // let length = dist.length();

        let a = G * earth.mass / (r * r);
        machine.vel += dir * a * dt;

        print!("\r{:?}", machine.vel);
        std::io::stdout().flush().unwrap();
        
        draw_circle(
            draw_earth_x as f32, 
            draw_earth_y as f32, 
            (earth.r * PIXELS_PER_METER) as f32, 
            BLUE
        );

        draw_fps();
        next_frame().await;
    }
}
