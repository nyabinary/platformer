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
    let mut gravity = 0.0;

    // gravity = 0
    // each frame, if gravity > -10, gravity -= 1
    // y += gravity

    // For jumping, when we press space, gravity = 15.0
    // For implmenting floor, check each frame if y > 640.0. if so, set gravity = 0

    loop {
        // Set background color
        clear_background(SKYBLUE);

        // Moving left and right
        if is_key_down(KeyCode::Right) {
            x += 3.0;
        }

        if is_key_down(KeyCode::Left) {
            x -= 3.0;
        }
        
        // Gravity
        if gravity > -10.0 {
            gravity -= 1.0;
        }
        
        // Floor
        if y > 530.0 {
            gravity = 0.0;
        }

        // Jumping
        if is_key_pressed(KeyCode::Space) {
            gravity = 15.0;
        }

        y -= gravity;

        // Draw all the objects
        draw_rectangle(x, y, 64.0, 64.0, GREEN);

        // Render next frame
        next_frame().await
    }
}
