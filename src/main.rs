use macroquad::prelude::*;

// [ ] Add gravity
// [ ] Add jump
// [ ] Add sprinting
// [ ] Add tiles to stand on

#[macroquad::main("BasicShapes")]
async fn main() {
    // Define state (which is the same as variables)
    let mut x = 0.0;
    let mut y = 0.0;

    loop {
        // Set background color
        clear_background(SKYBLUE);

        // Update state
        if is_key_down(KeyCode::Right) {
            x += 1.0;
        }
        if is_key_down(KeyCode::Left) {
            x -= 1.0;
        }
        if is_key_down(KeyCode::Up) {
            y -= 1.0;
        }
        if is_key_down(KeyCode::Down) {
            y += 1.0;
        }

        // Draw all the objects
        draw_rectangle(x, y, 64.0, 64.0, GREEN);

        // Render next frame
        next_frame().await
    }
}
