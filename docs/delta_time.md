# デルタタイムで速度を安定させる

フレームレートに依存させない

```rust
#[macroquad::main("")]
async fn main() {
    let mut x = 300.0f32;
    let speed = 200.0;

    loop {
        let dt = get_frame_time(); // 前フレームからの経過秒数
        clear_background(BLACK); // 前フレームをクリア

        // キー入力（押している間）
        if is_key_down(KeyCode::Right) { x += speed * dt; }
        if is_key_down(KeyCode::Left)  { x -= speed * dt; }

        draw_circle(x, 200.0, 200.0, WHITE);
        draw_text(&format!("FPS: {}", get_fps()), 10.0, 20.0, 20.0, WHITE);

        next_frame().await; // 忘れると無限ループ
    }
}
```
