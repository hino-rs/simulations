# テクスチャ

```rust
use macroquad::prelude::*;

#[macroquad::main("Texture")]
async fn main() {
    // ファイルから読み込み（非同期）
    let texture: Texture2D = load_texture("player.png").await.unwrap();

    loop {
        clear_background(BLACK);

        // シンプルに描画
        draw_texture(&texture, 100.0, 100.0, WHITE);

        // サイズ・回転・色を指定して描画
        draw_texture_ex(
            &texture,
            200.0, 200.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(64.0, 64.0)), // 64x64にリサイズ
                rotation: get_time() as f32,        // 時間で回転
                ..Default::default()
            },
        );

        next_frame().await;
    }
}
```
