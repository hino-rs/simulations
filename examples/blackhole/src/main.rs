use macroquad::prelude::*;
use std::collections::VecDeque;

const G: f64 = 6.67430e-11;
const C: f64 = 299792458.0;
const EARTH_M: f64 = 5.972e24; // 地球の質量
const EARTH_R: f64 = 6.371e6;  // 地球の半径
const PIXELS_PER_METER: f64 = 7.886e-9; // 画面上の表示スケール(1メートルを何ピクセルにするか)

const TRAIL_LENGTH: usize = 1024;

fn window_conf() -> Conf {
    Conf {
        window_title: "BlackHoleSimulation".to_owned(),
        window_width: 1920,
        window_height: 1080,
        fullscreen: true,
        ..Default::default()
    }
}

struct BlackHole {
    position: DVec2,
    mass: f64,
    r_s: f64, // r_s = (2 * G * M) / c^2    
}

impl BlackHole {
    fn new(pos: DVec2, m: f64) -> Self {
        Self {
            position: pos,
            mass: m,
            r_s: (2.0 * G * m) / (C * C),
        }
    }
}

struct Ray {
    x: f64,
    y: f64,
    dir: DVec2,
    r: f64,
    phi: f64,
    dr: f64,
    dphi: f64,
    d2r: f64,
    d2phi: f64,
    trail: VecDeque<Vec2>,
}

impl Ray {
    fn new(pos: DVec2, dir: DVec2) -> Self {
        let x = pos.x;
        let y = pos.y;

        let r = x.hypot(y);
        let phi = y.atan2(x);
        let dr = C * (dir.x * phi.cos() + dir.y * phi.sin());
        let dphi = C * (-dir.x * phi.sin() + dir.y * phi.cos()) / r;
        let d2r = 0.0;
        let d2phi = 0.0;

        Self {
            x,
            y,
            dir,
            r,
            phi,
            dr,
            dphi,
            d2r,
            d2phi,
            trail: VecDeque::with_capacity(TRAIL_LENGTH),
        }
    }
}

impl Ray {
    fn get_derivatives(&self, y: [f64; 4], r_s: f64) -> [f64; 4] {
        let r = y[0];
        let dr = y[2];
        let dphi = y[3];

        // 一般相対性理論における光の動径方向の加速度 (d2r)
        let d2r = (r - 1.5 * r_s) * dphi * dphi;
        // 角度方向の加速度 (d2phi)
        let d2phi = -2.0 * dr * dphi / r;

        [dr, dphi, d2r, d2phi]
    }

    fn step_rk4(&mut self, dt: f64, r_s: f64) {
        if self.r < r_s * 0.95 { return; }

        let y = [self.r, self.phi, self.dr, self.dphi];

        // k1 = f(y)
        let k1 = self.get_derivatives(y, r_s);

        // k2 = f(y + 0.5 * dt * k1)
        let mut y_temp = [0.0; 4];
        for i in 0..4 {
            y_temp[i] = y[i] + 0.5 * dt * k1[i];
        }
        let k2 = self.get_derivatives(y_temp, r_s);

        // k3 = f(y + 0.5 * dt * k2)
        for i in 0..4 {
            y_temp[i] = y[i] + 0.5 * dt * k2[i];
        }
        let k3 = self.get_derivatives(y_temp, r_s);

        // k4 = f(y + dt * k3)
        for i in 0..4 {
            y_temp[i] = y[i] + dt * k3[i];
        }
        let k4 = self.get_derivatives(y_temp, r_s);

        // y_next = y + (dt/6) * (k1 + 2*k2 + 2*k3 + k4)
        self.r    += (dt / 6.0) * (k1[0] + 2.0 * k2[0] + 2.0 * k3[0] + k4[0]);
        self.phi  += (dt / 6.0) * (k1[1] + 2.0 * k2[1] + 2.0 * k3[1] + k4[1]);
        self.dr   += (dt / 6.0) * (k1[2] + 2.0 * k2[2] + 2.0 * k3[2] + k4[2]);
        self.dphi += (dt / 6.0) * (k1[3] + 2.0 * k2[3] + 2.0 * k3[3] + k4[3]);

        // 直交座標 (x, y) に変換して反映
        self.x = self.phi.cos() * self.r;
        self.y = self.phi.sin() * self.r;
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let width = screen_width();
    let height = screen_height();
    let center = DVec2::new((width / 2.0) as f64, (height / 2.0) as f64);

    // xp = 画面の中心X + xm * PIXELS_PER_METER
    // yp = 画面の中心Y + ym * PIXELS_PER_METER
    let sag_a = BlackHole::new(
        DVec2::new(0.0, 0.0), 
        8.54e36
    );
    let draw_sag_a_x = center.x + sag_a.position.x * PIXELS_PER_METER;
    let draw_sag_a_y = center.y + sag_a.position.y * PIXELS_PER_METER;

    // let mut rays = Vec::new();
    // let num_rays = 1000;
    // for i in 0..num_rays {
    //     let y_pixel = -450.0 + (i as f64) * 900.0 / (num_rays - 1) as f64;
    //     let x = -center.x / PIXELS_PER_METER;
    //     let y = y_pixel / PIXELS_PER_METER;
    //     let ray = Ray::new(
    //         DVec2 {
    //             x,
    //             y: -center.y / PIXELS_PER_METER * 0.95,
    //         },
    //         DVec2 { x: 1.0, y: (i as f64).sin().abs() }
    //     );
    //     rays.push(ray);
    // }
    let mut rays = Vec::new();
    let num_rays = 500;
    for i in 0..num_rays {
        // 259.8ピクセルの臨界値の前後（255px〜265px）に光線を集中させる
        let y_pixel = -255.0 - (i as f64) * 10.0 / (num_rays - 1) as f64;
        
        let ray = Ray::new(
            DVec2 {
                x: -center.x / PIXELS_PER_METER,
                y: y_pixel / PIXELS_PER_METER,
            },
            DVec2 { x: 1.0, y: 0.0 }
        );
        rays.push(ray);
    }


    loop {
        let dt = get_frame_time() * 100.0;
        clear_background(Color::new(0.0, 0.0, 0.1, 1.0));

        // 全ての光線を更新・描画
        for ray in &mut rays {
            let draw_ray_x = (center.x + ray.x * PIXELS_PER_METER) as f32;
            let draw_ray_y = (center.y + ray.y * PIXELS_PER_METER) as f32;

            if ray.r >= sag_a.r_s * 0.95 {
                ray.trail.push_front(Vec2::new(draw_ray_x, draw_ray_y));
                if ray.trail.len() > TRAIL_LENGTH {
                    ray.trail.pop_back();
                }
                ray.step_rk4(dt as f64, sag_a.r_s);
            }

            // トレイルを描画
            for (i, p) in ray.trail.iter().enumerate() {
                let t = 1.0 - (i as f32 / TRAIL_LENGTH as f32);
                let radius = 2.0 * t;
                let alpha = t * 0.6;
                draw_circle(p.x, p.y, radius, Color::new(1.0, 1.0, 0.7, alpha));
            }

            // 現在位置（光の先端）を描画
            if ray.r >= sag_a.r_s * 0.95 {
                draw_circle(draw_ray_x, draw_ray_y, 3.0, Color::new(1.0, 1.0, 0.7, 1.0));
            }

            let mut x = DVec3::new(3.0, 8.0, -4.0);
            x = x.normalize();
        }

        // ブラックホールを描画
        draw_circle(
            draw_sag_a_x as f32, 
            draw_sag_a_y as f32, 
            (sag_a.r_s * PIXELS_PER_METER) as f32,
            BLACK,
        );

        next_frame().await;
    }
}
