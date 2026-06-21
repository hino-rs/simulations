# 文字

```rust
        // 基本
        draw_text("Hello, macroquad!", 20.0, 40.0, 32.0, WHITE);

        // 中央に配置したいとき
        let text = "Centered";
        let font_size = 40.0;
        let dims = measure_text(text, None, font_size as u16, 1.0);
        draw_text(
            text, 
            screen_width() / 2.0 - dims.width / 2.0, 
            screen_height() / 2.0, 
            font_size, 
            GREEN
        );
```
