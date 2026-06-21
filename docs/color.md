# 色

```rust
// 定義済み定数
let c1 = RED;
let c2 = BLUE;
let c3 = GREEN;
let c4 = WHITE;
let c5 = BLACK;
let c6 = YELLOW;
let c7 = DARKGRAY;

// RGB で指定（0.0〜1.0）
let c8 = Color::new(1.0, 0.5, 0.0, 1.0); // オレンジ (R, G, B, A)

// 16進数から
let c9 = color_u8!(255, 128, 0, 255); // マクロでu8指定
```
