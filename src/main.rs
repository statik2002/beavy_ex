use macroquad::prelude::*;

const DEBUG: bool = true;

fn window_conf() -> Conf {
    Conf {
        window_title: "First steps".to_owned(),
        fullscreen: true,
        ..Default::default()
    }
}

fn draw_axes(start: Vec3) {
    draw_line_3d(start, vec3(start.x + 3., start.y, start.z), RED); // X
    draw_line_3d(start, vec3(start.x, start.y, start.z + 3.), GREEN); // Y
    draw_line_3d(start, vec3(start.x, start.y + 3., start.z), BLUE); // Z
}

#[macroquad::main(window_conf)]
async fn main() {
    //block count
    const BLOCKS_W: usize = 10;
    const BLOCKS_H: usize = 5;
    // block width
    const WALL_W: f32 = 5.0;

    // bloks array
    let mut blocks: [[bool; BLOCKS_W]; BLOCKS_H] = [[true; BLOCKS_W]; BLOCKS_H];

    let mut platform_x = screen_width() / 2.;
    let mut platform_y_shift = 20.;
    let platform_width = 300.;
    let platform_height = 20.;
    let platform_velocity: f32 = 300.;

    let mut ball_x = screen_width() / 2.;
    let mut ball_y = screen_height() / 2.;
    let mut ball_radius: f32 = 15.;
    let mut dx = 200.0;
    let mut dy = 200.0;
    let mut stick = false;

    //let ferris = load_texture("tex2.png").await.unwrap();

    loop {
        clear_background(BLACK);
        
        let delta = get_frame_time();

        if is_key_down(KeyCode::Right) && platform_x < screen_width() - platform_width / 2. {
            platform_x += platform_velocity * delta;
        }

        if is_key_down(KeyCode::Left) && platform_x > 0. + platform_width / 2. {
            platform_x -= platform_velocity * delta;
        }

        if is_key_down(KeyCode::Escape) {
            break;
        }


        set_camera(&Camera3D{
            position: vec3(screen_width() / 2., screen_height() / 2. + 100., -1500.),
            up: vec3(0., -1., 0.),
            target: vec3(screen_width() / 2., screen_height() / 2., 0.),
            projection: Projection::Perspective,
            ..Default::default()
        });


        //draw_grid(20, 1., BLACK, GRAY);

        // Draw blocks
        let block_width = screen_width() / BLOCKS_W as f32 - 20.;
        let block_height = screen_height() / BLOCKS_H as f32 / 5.;
        let shift: f32 = (BLOCKS_W as f32 * block_width - screen_width()) + 300.;
        for j in 0..BLOCKS_H{
            for i in 0..BLOCKS_W {
                if blocks[j][i] {
                    let block_x = i as f32 * block_width + shift;
                    let block_y = j as f32 * block_height + 50.;
                    draw_cube_wires(vec3(block_x, block_y, 0.), vec3(block_width, block_height, 50.), WHITE);
                }
            }
        }
        // Calculate collisions
        'outer: for j in 0..BLOCKS_H{
            for i in 0..BLOCKS_W {
                if blocks[j][i] {
                    let block_x = i as f32 * block_width + shift;
                    let block_y = j as f32 * block_height + 50.;

                    if ball_y >= block_y - block_height / 2. - ball_radius && ball_y <= block_y + block_height / 2. + ball_radius && ball_x >= block_x - block_width / 2. - ball_radius && ball_x <= block_x + block_width / 2. + ball_radius {
                        let cx = ball_x - block_x;
                        let cy = ball_y - block_y;
                        if cx <= 0. && cy <= 0. {
                            // left up quater
                            if ball_x <= block_x - block_width / 2. {
                                dx *= -1.;
                                blocks[j][i] = false;
                                break 'outer;
                            } else {
                                dy *= -1.;
                                blocks[j][i] = false;
                                break 'outer;
                            }
                        }
                        if cx > 0. && cy < 0. {
                            // right up quater
                            if ball_x >= block_x + block_width / 2. {
                                dx *= -1.;
                                blocks[j][i] = false;
                                break 'outer;
                            } else {
                                dy *= -1.;
                                blocks[j][i] = false;
                                break 'outer;
                            }
                        }
                        if cx < 0. && cy > 0. {
                            // left bottom quater
                            if ball_x <= block_x - block_width / 2. {
                                dx *= -1.;
                                blocks[j][i] = false;
                                break 'outer;
                            } else {
                                dy *= -1.;
                                blocks[j][i] = false;
                                break 'outer;
                            }
                        }
                        if cx >= 0. && cy >= 0. {
                            // right bottom quater
                            if ball_x >= block_x + block_width /2. {
                                dx *= -1.;
                                blocks[j][i] = false;
                                break 'outer;
                            } else {
                                dy *= -1.;
                                blocks[j][i] = false;
                                break 'outer;
                            }
                        }
                    }
                }
            }
        }

        // draw walls
        draw_cube_wires(vec3(WALL_W, screen_height() / 2., 0.), vec3(WALL_W, screen_height(), 50.), WHITE); // left wall
        draw_cube_wires(vec3(screen_width() - WALL_W, screen_height() / 2. + 1., 0.), vec3(WALL_W, screen_height(), 50.), WHITE); // right wal
        draw_cube_wires(vec3(screen_width() / 2., WALL_W / 2. + 1., 0.), vec3(screen_width() - 1., WALL_W, 50.), WHITE); // top wall

        // Claculate walls colisions
        if ball_x <= 0. + ball_radius + WALL_W / 2. || ball_x > screen_width() - ball_radius - WALL_W / 2. {
            dx *= -1.;
        }
        if ball_y >= screen_height() - platform_y_shift - platform_height / 2. - ball_radius  
            && ball_x >= platform_x - platform_width / 2. && ball_x <= platform_x + platform_width / 2. {
            dy *= -1.;
        }
        if ball_y >= screen_height() || ball_y < 0. + ball_radius + WALL_W / 2.{
            //ball_y = 10.;
            dy *= -1.;
            //stick = true;
        }
        if stick == false {
            ball_x += dx * delta;
            ball_y += dy * delta;
        }

        // draw ball
        draw_sphere_wires(vec3(ball_x, ball_y, 0.), ball_radius, None, WHITE);

        //draw pad
        draw_cube_wires(
            vec3(platform_x, screen_height() - platform_y_shift, 0.),
            vec3(platform_width, platform_height, 0.),
            WHITE
        );

        set_default_camera();
        
        // Output debug information
        if DEBUG {
            draw_text("WELCOME TO 3D WORLD", 10.0, 20.0, 30.0, WHITE);
            draw_axes(vec3(-10., 0., -15.));
            let fps = get_fps();
            draw_text(&format!("FPS: {}", fps), 10.0,50.0, 30.0, WHITE);
            draw_text(&format!("Platform x: {}, Y {}", platform_x, platform_y_shift), 10.0,70.0, 30.0, WHITE);
            draw_text(&format!("Ball X: {}, Z: {}", ball_x, ball_y), 10.0,100.0, 30.0, WHITE);
            draw_text("X", 10.0,120.0, 20.0, RED);
            draw_text("Y", 25.0,120.0, 20.0, BLUE);
            draw_text("Z", 40.0,120.0, 20.0, GREEN);
            draw_text(&format!("Screen width: {}, Screen height: {}", screen_width(), screen_height()), 10.0,150.0, 20.0, GREEN);
            draw_text(&format!("Block shift: {}", shift), 10.0,170.0, 20.0, WHITE);
        }

        next_frame().await
    }
}