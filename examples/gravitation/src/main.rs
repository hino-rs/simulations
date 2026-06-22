use macroquad::prelude::*;

const G: f64 = 6.674e-11;
const PIXELS_PER_METER: f64 = 1.0e-5;

// const EARTH_M: f64 = 5.972e24;
const EARTH_M: f64 = 5.972e29;

#[derive(Debug)]
struct Object {
    mass: f64,
    pos: DVec3,
    vel: DVec3,
    r: f64,
    color: Color,
}

impl Object {
    fn draw(&self, center: DVec3) {
        let draw_x = (center.x + self.pos.x * PIXELS_PER_METER) as f32;
        let draw_y = (center.y + self.pos.y * PIXELS_PER_METER) as f32;
        let draw_z = (center.z + self.pos.z * PIXELS_PER_METER) as f32;

        draw_sphere(
            Vec3::new(draw_x, draw_y, draw_z),
            (self.r * PIXELS_PER_METER) as f32,
            None,
            self.color,
        );
    }
}

#[macroquad::main("")]
async fn main() {
    let width = screen_width() as f64;
    let height = screen_height() as f64;
    let center = DVec3::new(width / 2.0, height / 2.0, 0.0);

    let mut simulation_speed = 1.0;

    let mut earth = Object {
        mass: EARTH_M,
        pos: DVec3::ZERO,
        vel: DVec3::ZERO,
        r: 6.371e6,
        color: BLUE,
    };

    let rock_r = earth.r / 10.0;
    let mut rock = Object {
        // mass: 10e3, // 1T
        mass: EARTH_M * 0.5,
        pos: DVec3 { x: 0.0, y: earth.r+rock_r*1.05, z: 0.0 },
        vel: DVec3 { x: 0.0, y: 0.0, z: 0.0 },
        r: rock_r,
        color: WHITE,
    };

    let mut camera_zoom = 1000.0f32;
    let mut camera_yaw = 0.0f32;
    let mut camera_pitch = 0.0f32;

    loop {
        let dt = get_frame_time() as f64 * simulation_speed;
        clear_background(BLACK);

        // シミュレーション速度
        if is_key_down(KeyCode::Up) { simulation_speed += 1.0; }
        if is_key_down(KeyCode::Down) { simulation_speed -= 1.0; }
        if is_key_down(KeyCode::Right) { simulation_speed *= 2.0; }
        if is_key_down(KeyCode::Left) { simulation_speed /= 2.0; }

        // カメラのズーム（マウスホイール）
        let wheel = mouse_wheel();
        if wheel.1 != 0.0 {
            camera_zoom -= wheel.1 * 100.0;
            camera_zoom = camera_zoom.clamp(100.0, 5000.0);
        }

        // カメラの回転（左マウスドラッグ）
        if is_mouse_button_down(MouseButton::Left) {
            let delta = mouse_delta_position();
            camera_yaw += delta.x * 3.0;
            camera_pitch += delta.y * 3.0;
            camera_pitch = camera_pitch.clamp(-1.4, 1.4);
        }

        // カメラの位置を計算（軌道カメラ）
        let cos_pitch = camera_pitch.cos();
        let sin_pitch = camera_pitch.sin();
        let cos_yaw = camera_yaw.cos();
        let sin_yaw = camera_yaw.sin();

        let camera_offset = vec3(
            camera_zoom * cos_pitch * sin_yaw,
            camera_zoom * sin_pitch,
            camera_zoom * cos_pitch * cos_yaw,
        );
        let camera_pos = vec3(center.x as f32, center.y as f32, 0.0) + camera_offset;

        set_camera(&Camera3D {
            position: camera_pos,
            target: vec3(center.x as f32, center.y as f32, 0.0),
            up: vec3(0.0, 1.0, 0.0),
            ..Default::default()
        });

        // カメラの前方向ベクトル
        let forward = -camera_offset.normalize();
        // カメラの右方向ベクトル
        let right = forward.cross(vec3(0.0, 1.0, 0.0)).normalize();
        //　カメラの上方向ベクトル
        let up = right.cross(forward).normalize();
        let forward_d = DVec3::new(forward.x as f64, forward.y as f64, forward.z as f64);
        let right_d   = DVec3::new(right.x as f64,   right.y as f64,   right.z as f64);
        let up_d      = DVec3::new(up.x as f64,  up.y as f64,  up.z as f64);

        // 基準面のグリッドを描画
        let grid_size = 1000.0f32;
        let grid_steps = 10;
        for i in -grid_steps..=grid_steps {
            let offset = i as f32 * (grid_size / grid_steps as f32);
            // 横線 (XY平面)
            draw_line_3d(
                vec3(center.x as f32 - grid_size, center.y as f32 + offset, 0.0),
                vec3(center.x as f32 + grid_size, center.y as f32 + offset, 0.0),
                GRAY,
            );
            // 縦線 (XY平面)
            draw_line_3d(
                vec3(center.x as f32 + offset, center.y as f32 - grid_size, 0.0),
                vec3(center.x as f32 + offset, center.y as f32 + grid_size, 0.0),
                GRAY,
            );
        }

        // --- 岩の操作 ---
        let speed = 10.0;
        if is_key_down(KeyCode::W) { rock.vel += forward_d * speed; }
        if is_key_down(KeyCode::S) { rock.vel -= forward_d * speed; }
        if is_key_down(KeyCode::A) { rock.vel -= right_d * speed; }
        if is_key_down(KeyCode::D) { rock.vel += right_d * speed; }
        if is_key_down(KeyCode::Q) { rock.vel -= up_d * speed; }
        if is_key_down(KeyCode::E) { rock.vel += up_d * speed; }


        // --- 物理計算 ---
        let dist_m = earth.pos - rock.pos;
        let r = dist_m.length();
        let dir = dist_m.normalize();
        let a = G * earth.mass / (r * r);
        rock.vel += dir * a * dt;

        // 座標に速度加算
        earth.pos += earth.vel * dt * PIXELS_PER_METER;
        rock.pos += rock.vel * dt * PIXELS_PER_METER;

        // 衝突
        if rock.pos.length()-rock.r < earth.r  {
            // --- 座標の正規化 ---
            let d = rock.pos - earth.pos;
            let n = d.normalize();
            // Ce + (Re + Rr) * n で地表ぴったりに移動させる
            rock.pos = earth.pos + (earth.r + rock.r) * n;
            
            // --- 速度の影響 ---
            // 地面に突き刺さる垂直方向のスピードを取り出す
            let v_normal = rock.vel.dot(n);

            // 地面に向かっている場合だけ処理する
            if v_normal < 0.0 {
                // 地面にぶつかる速度を取り除き、さらに 80% で跳ね返す
                rock.vel -= n * (v_normal * 1.8);
            }
        }

        // 描画
        earth.draw(center);
        rock.draw(center);

        // テキスト描画
        set_default_camera();
        
        draw_fps();

        let vel_text = format!("Rock Speed: ({:.1}km, {:.1}km, {:.1}km)", rock.vel.x / 1000.0, rock.vel.z / 1000.0, rock.vel.z / 1000.0);
        let position_text_dims = measure_text(&vel_text, None, 32, 1.0);
        draw_text(
            &vel_text, 
            screen_width() / 2.0 - position_text_dims.width / 2.0, 
            screen_height() * 0.9, 
            32.0, 
            WHITE
        );
        
        draw_text(format!("Simulation Speed: {:.0}", simulation_speed), 10.0, 50.0, 32.0, WHITE);
        draw_text(format!("Distance from Rock to Earth: {:.0}km", rock.pos.distance(earth.pos)), 10.0, 75.0, 32.0, WHITE);
        next_frame().await;
    }
}
