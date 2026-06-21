use macroquad::prelude::*;

const G: f64 = 6.674e-11;
// const EARTH_M: f64 = 5.972e24;
const EARTH_M: f64 = 5.972e30;
// const EARTH_M: f64 = 8.54e36;
const EARTH_R: f64 = 6.371e6;
const PIXELS_PER_METER: f64 = 1.0e-5;
const SIMULATION_SPEED: f64 = 100.0;

struct Spacecraft {
    pos: DVec3,
    angle: f64,
    jet_vel: f64,
    vel: DVec3,    
    mass: f64,
}

struct Planet {
    pos: DVec3,
    mass: f64,
    r: f64,
}

impl Spacecraft {
    fn draw3d(&self, x: f64, y: f64, z: f64) {
        let size = vec3(30.0, 10.0, 10.0);
        let angle = self.angle as f32;
        let center = vec3(x as f32, y as f32, z as f32);
        let color = WHITE;

        let hw = size.x / 2.0;
        let hh = size.y / 2.0;
        let hd = size.z / 2.0;

        // Z軸回転（XY平面内の回転。進行方向と一致させる）
        let cos = angle.cos();
        let sin = angle.sin();
        let rotate = |x: f32, y: f32, z: f32| -> Vec3 {
            vec3(
                center.x + x * cos - y * sin,
                center.y + x * sin + y * cos,
                center.z + z,
            )
        };

        // 8頂点
        let v = [
            rotate(-hw, -hh, -hd), // 0: 左下前
            rotate( hw, -hh, -hd), // 1: 右下前
            rotate( hw,  hh, -hd), // 2: 右上前
            rotate(-hw,  hh, -hd), // 3: 左上前
            rotate(-hw, -hh,  hd), // 4: 左下後
            rotate( hw, -hh,  hd), // 5: 右下後
            rotate( hw,  hh,  hd), // 6: 右上後
            rotate(-hw,  hh,  hd), // 7: 左上後
        ];

        // 12本の辺を描画（本体のワイヤーフレーム）
        // 前面 (z = -hd)
        draw_line_3d(v[0], v[1], color);
        draw_line_3d(v[1], v[2], color);
        draw_line_3d(v[2], v[3], color);
        draw_line_3d(v[3], v[0], color);

        // 後面 (z = hd)
        draw_line_3d(v[4], v[5], color);
        draw_line_3d(v[5], v[6], color);
        draw_line_3d(v[6], v[7], color);
        draw_line_3d(v[7], v[4], color);

        // 前面と後面を繋ぐ辺
        draw_line_3d(v[0], v[4], color);
        draw_line_3d(v[1], v[5], color);
        draw_line_3d(v[2], v[6], color);
        draw_line_3d(v[3], v[7], color);

        // 機首のコーン（先端）を追加して進行方向を分かりやすくする
        let nose_tip = rotate(hw + 10.0, 0.0, 0.0);
        draw_line_3d(nose_tip, v[1], RED); // 右下前
        draw_line_3d(nose_tip, v[2], RED); // 右上前
        draw_line_3d(nose_tip, v[5], RED); // 右下後
        draw_line_3d(nose_tip, v[6], RED); // 右上後

        // エンジンのノズルを追加
        let nozzle_tip = rotate(-hw - 5.0, 0.0, 0.0);
        draw_line_3d(nozzle_tip, v[0], ORANGE); // 左下前
        draw_line_3d(nozzle_tip, v[3], ORANGE); // 左上前
        draw_line_3d(nozzle_tip, v[4], ORANGE); // 左下後
        draw_line_3d(nozzle_tip, v[7], ORANGE); // 左上後
    }

    fn jet(&mut self, dt: f64) {
        let delta_x = self.jet_vel * self.angle.cos();
        let delta_y = self.jet_vel * self.angle.sin();
        
        self.vel.x += delta_x * dt;
        self.vel.y += delta_y * dt;
    }

    fn stop(&mut self) {
        self.vel = DVec3::ZERO;
    }
}

fn linear_motion(machine: &mut Spacecraft, dt: f64) {
    // let delta_x = machine.vel.x * machine.angle.cos();
    // let delta_y = machine.vel.y * machine.angle.sin();

    machine.pos.x += machine.vel.x * dt * PIXELS_PER_METER;
    machine.pos.y += machine.vel.y * dt * PIXELS_PER_METER;
    machine.pos.z += machine.vel.z * dt * PIXELS_PER_METER;
}

#[macroquad::main("")]
async fn main() {
    let width = screen_width() as f64;
    let height = screen_height() as f64;
    let center = DVec3::new(width / 2.0, height / 2.0, 0.0);

    let mut earth = Planet {
        pos: DVec3::ZERO,
        mass: EARTH_M,
        r: EARTH_R,
    };

    let mut machine = Spacecraft {
        pos: DVec3 { 
            // x: -(EARTH_R + 400e3),
            x: -(EARTH_R + 16_000_000.0),
            y: 0.0,
            z: 0.0,
        },
        angle: 0.0,
        jet_vel: 100.0,
        vel: DVec3::ZERO,
        mass: 2800.0 * 1000.0,
    };

    let mut camera_zoom = 1000.0f32;
    let mut camera_yaw = 0.0f32;
    let mut camera_pitch = 0.0f32;

    loop {
        let dt = get_frame_time() as f64 * SIMULATION_SPEED; // 前フレームからの経過秒数
        clear_background(BLACK); // 前フレームをクリア

        // get_time() (秒単位) を用いてZ軸方向に緩やかに往復運動させる (周期: 約31.4秒, 振幅: 2000万メートル = 画面上200px)
        let time = get_time();
        earth.pos = DVec3 {
            x: 0.0,
            y: 0.0,
            z: (time * 0.2).sin() * 20_000_000.0,
        };

        let draw_earth_x = center.x + earth.pos.x * PIXELS_PER_METER;
        let draw_earth_y = center.y + earth.pos.y * PIXELS_PER_METER;
        let draw_earth_z = center.z + earth.pos.z * PIXELS_PER_METER;

        // std::io::stdout().flush().unwrap();

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

        // --- 3D描画パス ---
        set_camera(&Camera3D {
            position: camera_pos,
            target: vec3(center.x as f32, center.y as f32, 0.0),
            up: vec3(0.0, 1.0, 0.0),
            ..Default::default()
        });

        // 基準面のグリッドを描画
        let grid_size = 1000.0f32;
        let grid_steps = 10;
        for i in -grid_steps..=grid_steps {
            let offset = i as f32 * (grid_size / grid_steps as f32);
            // 横線 (XY平面)
            draw_line_3d(
                vec3(center.x as f32 - grid_size, center.y as f32 + offset, 0.0),
                vec3(center.x as f32 + grid_size, center.y as f32 + offset, 0.0),
                DARKGRAY,
            );
            // 縦線 (XY平面)
            draw_line_3d(
                vec3(center.x as f32 + offset, center.y as f32 - grid_size, 0.0),
                vec3(center.x as f32 + offset, center.y as f32 + grid_size, 0.0),
                DARKGRAY,
            );
        }

        // 3D地球（経緯線入りのワイヤーフレーム球）を描画
        draw_sphere_wires(
            vec3(draw_earth_x as f32, draw_earth_y as f32, draw_earth_z as f32),
            (earth.r * PIXELS_PER_METER) as f32,
            None,
            BLUE,
        );

        // 宇宙船を描画
        let draw_machine_x = center.x + machine.pos.x * PIXELS_PER_METER;
        let draw_machine_y = center.y + machine.pos.y * PIXELS_PER_METER;
        let draw_machine_z = center.z + machine.pos.z * PIXELS_PER_METER;
        machine.draw3d(draw_machine_x, draw_machine_y, draw_machine_z);

        // --- 物理演算 & キー入力処理 ---
        if is_key_pressed(KeyCode::E) {
            machine.jet_vel *= 2.0;
        }
        if is_key_pressed(KeyCode::Q) {
            machine.jet_vel /= 2.0;
        }

        if is_key_down(KeyCode::Left) {
            machine.angle += 3.0 * (dt / SIMULATION_SPEED);
        }
        if is_key_down(KeyCode::Right) {
            machine.angle -= 3.0 * (dt / SIMULATION_SPEED);
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

        let a = G * earth.mass / (r * r);
        machine.vel += dir * a * dt;

        // print!("\r{:?}", machine.vel);
        // std::io::stdout().flush().unwrap();

        // --- 2D描画パス (HUDやテキスト表示) ---
        set_default_camera();

        let position_text = format!("Position: ({:.1}km, {:.1}km, {:.1}km)", machine.pos.x, machine.pos.y, machine.pos.z);
        let position_text_dims = measure_text(&position_text, None, 32, 1.0);
        draw_text(
            &position_text, 
            screen_width() / 2.0 - position_text_dims.width / 2.0, 
            screen_height() * 0.9, 
            32.0, 
            WHITE
        );

        // カメラ操作ヘルプ
        draw_text("Drag Left Mouse Button to Orbit Camera", 20.0, 40.0, 20.0, GRAY);
        draw_text("Scroll Mouse Wheel to Zoom In/Out", 20.0, 65.0, 20.0, GRAY);
        draw_text("Space: Jet Engine | Left Ctrl: Stop Ship | Arrow Keys: Rotate Ship", 20.0, 90.0, 20.0, GRAY);

        draw_fps();
        next_frame().await;
    }
}
