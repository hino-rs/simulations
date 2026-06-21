# 設定

```rust
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "My Game".to_owned(),
        window_width: 1280,
        window_height: 720,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    loop {
        clear_background(BLACK);
        draw_text(
            &format!("{}x{}", screen_width(), screen_height()),
            20.0, 20.0, 24.0, WHITE
        );
        next_frame().await;
    }
}
```
