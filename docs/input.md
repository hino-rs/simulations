# 入力

```rust
#[macroquad::main("")]
async fn main() {
    let mut x = 300.0f32;
    let mut y = 200.0f32;

    loop {
        clear_background(BLACK); // 前フレームをクリア

        // キー入力（押している間）
        if is_key_down(KeyCode::Right) { x += 3.0; }
        if is_key_down(KeyCode::Left)  { x -= 3.0; }
        if is_key_down(KeyCode::Down)  { y += 3.0; }
        if is_key_down(KeyCode::Up)    { y -= 3.0; }

        // マウスの座標
        let (mx, my) = mouse_position();

        // マウスクリック (押した瞬間だけ)
        if is_mouse_button_pressed(MouseButton::Left) {
            x = mx;
            y = my;
        }

        // 押してる間
        if is_mouse_button_down(MouseButton::Left) {
            x = mx;
            y = my;
        }

        draw_circle(x, y, 200.0, WHITE);

        next_frame().await; // 忘れると無限ループ
    }
}
```
