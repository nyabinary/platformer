use macroquad::prelude::*;

// [x] Add gravity
// [x] Add jump
// [x] Add sprinting
// [x] Add tiles to stand on
// [ ] Add collision detection

#[macroquad::main("BasicShapes")]
async fn main() {
    // Define state (which is the same as variables)
    // Create a player instance
    let mut player = Player {
        x: 0.0,
        y: 0.0,
        width: 64.0,
        height: 64.0,
    };
    let mut gravity = 0.0;
    let mut speed = 0.0;

    // When we press LeftShift, cube should move faster

    // gravity = 0
    // each frame, if gravity > -10, gravity -= 1
    // y += gravity

    // For jumping, when we press space, gravity = 15.0
    // For implmenting floor, check each frame if y > 640.0. if so, set gravity = 0

    // Add some tiles
    // 1. Create struct Tile with x: f32, y: f32, width: f32, height: f32,
    // 1. Create a vector of tiles
    // 2. Then we need to use for loop inside the loop, and draw each tile
    struct Tile {
        x: f32,
        y: f32,
        height: f32,
        width: f32,
    }

    struct Player {
        x: f32,
        y: f32,
        height: f32,
        width: f32,
    }

    // Create a new function for Tile so that we can call Tile::new(0.0, 0.0, 32.0, 32.0)
    impl Tile {
        fn new(x: f32, y: f32, height: f32, width: f32) -> Tile {
            Tile {
                x,
                y,
                height,
                width,
            }
        }
    }

    // Create a function called is_colliding, which takes
    fn is_colliding(player: &Player, tile: &Tile) -> bool {
        player.x < tile.x + tile.width
            && player.x + player.width > tile.x
            && player.y < tile.y + tile.height
            && player.y + player.height > tile.y
    }

    // Create four tiles inside a vec![], and bind it to tiles
    let tiles = vec![
        Tile::new(64.0 * 3.0, 196.0, 64.0, 64.0),
        Tile::new(64.0 * 4.0, 196.0, 64.0, 64.0),
        Tile::new(64.0 * 5.0, 196.0, 64.0, 64.0),
        Tile::new(64.0 * 6.0, 196.0, 64.0, 64.0),
    ];

    loop {
        // ~/.cargo/bin/bacon
        // Set background color
        clear_background(SKYBLUE);

        // If LeftShift, speed = 10.0 else speed = 3.0
        if is_key_down(KeyCode::LeftShift) {
            speed = 10.0;
        } else {
            speed = 3.0;
        }

        // Moving left and right
        if is_key_down(KeyCode::Right) {
            player.x += speed;
        }

        if is_key_down(KeyCode::Left) {
            player.x -= speed;
        }

        // Gravity
        if gravity > -10.0 {
            gravity -= 1.0;
        }

        // Floor
        if player.y > 530.0 {
            gravity = 0.0;
        }

        // Jumping
        if is_key_pressed(KeyCode::Space) {
            gravity = 15.0;
        }

        player.y -= gravity;

        // Collision
        // for tile in &tiles { if collision, move player back }
        for tile in &tiles {
            if is_colliding(&player, &tile) {
                // Do something with player
                player.x -= speed;
                player.y += gravity;
            }
        }

        // Draw all the objects
        draw_rectangle(player.x, player.y, 64.0, 64.0, GREEN);

        // Use for loop to loop over each vectortiles, then draw it
        for tile in &tiles {
            draw_rectangle(tile.x, tile.y, tile.width, tile.height, RED);
        }

        // Render next frame
        next_frame().await
    }
}
