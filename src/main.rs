

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
    const SCR_W: f32 = 20.0;
    const SCR_H: f32 = 20.0;

    let mut blocks: [[bool; BLOCKS_W]; BLOCKS_H] = [[true; BLOCKS_W]; BLOCKS_H];

    let mut platform_y = 0.;
    let platform_width = 5.;
    let platform_height = 0.2;

    let mut ball_x = 0.;
    let mut ball_y = -7.;
    let mut dx = 2.0;
    let mut dy = 2.0;
    let mut stick = false;
    //let ferris = load_texture("tex2.png").await.unwrap();

    loop {
        clear_background(BLACK);
        

        let delta = get_frame_time();

        if is_key_down(KeyCode::Right) && platform_y < 10.5 - platform_width / 2. {
            platform_y += 3.0 * delta;
        }

        if is_key_down(KeyCode::Left) && platform_y > -10.5 + platform_width / 2. {
            platform_y -= 3.0 * delta;
        }

        if is_key_down(KeyCode::Escape) {
            break;
        }

        
        set_camera(&Camera3D{
            position: vec3(0., 0., 40.),
            up: vec3(1., 0., 0.),
            target: vec3(0., 0., 0.),
            ..Default::default()
        });

        //draw_grid(20, 1., BLACK, GRAY);
        /* 
        draw_grid_ex(
            40, 
            0.5,
            DARKGRAY,
            DARKGRAY, 
            vec3(0., 0., 0.), 
            Quat{x: 180.,  y: 180., z: 0., w: 1.}
        );
        */

        /* 

        //draw_grid(20, 1., BLACK, GRAY);
        draw_cube_wires(vec3(0., 1., -6.), vec3(2., 2., 2.), DARKGREEN);
        draw_cube_wires(vec3(0., 1., 6.), vec3(2., 2., 2.), DARKBLUE);
        draw_cube_wires(vec3(2., 1., 2.), vec3(2., 2., 2.), YELLOW);

        //draw_plane(vec3(-8., 0., -8.), vec2(5., 5.), Some(&ferris), WHITE);

        draw_cube(
            vec3(-5., 1., -2.),
            vec3(2., 2., 2.),
            Some(&ferris),
            WHITE,
        );
        draw_cube(vec3(2., 0., -2.), vec3(1.4, 1.4, 1.4), None, GREEN);

        draw_sphere(vec3(-8., 0., 0.), 1., None, BLUE);
        draw_sphere_wires(vec3(-4., 0., 0.), 1., None, WHITE);

        */

        //draw blocks
        for j in 0..BLOCKS_H{
            for i in 0..BLOCKS_W {
                if blocks[j][i] {
                    let block_width = 20.0 / BLOCKS_W as f32;
                    let block_height = 10. / BLOCKS_H as f32;
                    let block_x = j as f32 * block_height + 1.5;
                    let block_y = i as f32 * block_width - 9.0;
                    draw_cube_wires(vec3(block_x, block_y, 0.), vec3(block_height, block_width, 0.5), WHITE);

                    if ball_y >= block_x - block_height / 2. && ball_y <= block_x + block_height / 2.  && ball_x >= block_y - block_width / 2. && ball_x <= block_y + block_width / 2. {
                        dy *= -1.;
                        blocks[j][i] = false;
                    }
                }
            }
        }
        //draw_cube_wires(vec3(10., 0., -10.), vec3(1., 1., 1.), WHITE);

        // draw walls
        draw_cube_wires(vec3(0., -10.5, 0.), vec3(21., 1., 0.5), WHITE);
        draw_cube_wires(vec3(0., 10.5, 0.), vec3(21., 1., 0.5), WHITE);
        draw_cube_wires(vec3(11., 0., 0.), vec3(1., 22., 0.5), WHITE);

        if ball_x <= -10. || ball_x > 10. {
            dx *= -1.;
        }

        // draw ball
        if ball_y <= -10.5 + platform_height + 0.2 && ball_x >= platform_y - platform_width / 2. && ball_x <= platform_y + platform_width / 2. {
            dy *= -1.;
        }
        if ball_y >= 10.5 {
            //ball_y = 10.;
            dy *= -1.;
            //stick = true;
        }
        if ball_y < -10.5 {
            dy *= -1.;
        }
        if stick == false {
            ball_x += dx * delta;
            ball_y += dy * delta;
        }
        draw_sphere_wires(vec3(ball_x, ball_y, 0.), 0.2, None, WHITE);
        //draw pad
        draw_cube_wires(
            vec3(-10.5, platform_y, 0.), 
            vec3(platform_height, platform_width, 0.), 
            WHITE
        );

        draw_axes(vec3(-10., 0., -15.));

        set_default_camera();
        draw_text("WELCOME TO 3D WORLD", 10.0, 20.0, 30.0, WHITE);
        if DEBUG {
            let fps = get_fps();
            draw_text(&format!("FPS: {}", fps), 10.0,50.0, 30.0, WHITE);
            draw_text(&format!("Platform Z: {},", platform_y), 10.0,70.0, 30.0, WHITE);
            draw_text(&format!("Ball X: {}, Z: {}", ball_x, ball_y), 10.0,100.0, 30.0, WHITE);
            draw_text("X", 10.0,120.0, 20.0, RED);
            draw_text("Y", 25.0,120.0, 20.0, BLUE);
            draw_text("Z", 40.0,120.0, 20.0, GREEN);
        }

        

        next_frame().await
    }
}