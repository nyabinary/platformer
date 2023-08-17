use macroquad::prelude::*;

// [x] Add gravity
// [x] Add jump
// [x] Add sprinting
// [x] Add tiles to stand on
// [x] Add collision detection

// Create an enum called Direction
enum Direction {
    Left,
    Right,
    Idle, 
}

#[macroquad::main("BasicShapes")]
async fn main() {
    // Define state (which is the same as variables)
    // Create a player instance
    let mut player = Player {
        x: 0.0,
        y: 0.0,
        width: 64.0,
        height: 64.0,
        direction: Direction::Idle,
    };
    let mut gravity = 0.0;
    let mut speed = 0.0;
    let mut counter: f32 = 0.0;

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
        direction: Direction,
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

    // We need to split is_colliding function into is_colliding_vertical and is_colliding_horizontal
    // We need to pass x, y, width, height and tile
    // This way, we can avoid copying by passing variables directly
    fn is_colliding(x: f32, y: f32, width: f32, height: f32, tile: &Tile) -> bool {
        y < tile.y + tile.height && y + height > tile.y && x < tile.x + tile.width && x + width > tile.x
    }

    // Create four tiles inside a vec![], and bind it to tiles
    let mut tiles = vec![
        Tile::new(64.0 * 3.0, 196.0, 64.0, 64.0),
        Tile::new(64.0 * 4.0, 196.0, 64.0, 64.0),
        Tile::new(64.0 * 5.0, 196.0, 64.0, 64.0),
        Tile::new(64.0 * 6.0, 196.0, 64.0, 64.0),
    ];

    loop {
        counter += 0.05;

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
            player.direction = Direction::Right;
        } else if is_key_down(KeyCode::Left) {
            player.direction = Direction::Left;
        } else {
            player.direction = Direction::Idle;
        }


        // Gravity
        if gravity > -10.0 {
            gravity -= 1.0;
        }

        // Jumping
        if is_key_pressed(KeyCode::Space) {
            gravity = 15.0;
        }

        // Match on player.direction, and modify player.x based on direction
        match player.direction {
            // <case> => <what to do>
            Direction::Left => player.x -= speed,
            Direction::Right => player.x += speed,
            Direction::Idle => {},
        }
        player.y -= gravity;

        // For each tile, move tile.x by counter.sin()


        // Collision
        // for tile in &tiles { if collision, move player back }
        for tile in &mut tiles {
            // We need to check if the player will collide with the tile in the next frame,
            // and if it will collide, we do these things:
            // - if it's vertically, move player back by gravity
            // - if it's horizontally, move player back by speed

            // To check vertically, run the is_colliding function with player.y + gravity
            if is_colliding(player.x, player.y - gravity, player.width, player.height, tile) {
                // Calculate delta between player.y + player.height and tile.y, then move that difference
                let delta = player.y - tile.y;
                // If delta is negative, the player is above the tile
                // If it's positive, the player is below the tile
                if delta < 0.0 {
                    player.y = tile.y - player.height; 
                } else {
                    player.y = tile.y + tile.height;
                }
                gravity = 0.0;
            }

            // To check horizontally, run the is_colliding function with player.x + speed
            let direction_value = match player.direction {
                Direction::Left => -1.0,
                Direction::Right => 1.0,
                Direction::Idle => 0.0,
            };
            
            if is_colliding(player.x + direction_value * speed, player.y, player.width, player.height, tile) {
                let delta = player.x - tile.x;
                // If delta is negative, tile is at the right.
                // If it's positive, tile is at the left.
                if delta < 0.0 {
                    player.x = tile.x - player.width;
                } else {
                    player.x = tile.x + tile.width;
                }
            }

            // Move tile.x by counter.sin()
            tile.x += counter.sin() * 5.0;
        }

        // sin, cos, tan

        // Floor
        if player.y > 536.0 {
            player.y = 536.0;
        }

        // If player.y < 0.0, move it to 0.0
        if player.y < 0.0 {
            player.y = 0.0;
        }

        if player.x < 0.0 {
            player.x = 0.0;
        }

        if player.x > 736.0 {
            player.x = 736.0;
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
